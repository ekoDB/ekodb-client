from typing import TypeVar, Dict, List, Optional, Union, Any
import requests
import pydantic
from .types import (
    TypedValue, ClientConfig, FindOptions, BatchInsertResponse,
    BatchUpdateResponse, BatchDeleteResponse, CollectionSchema,
    TokenResponse
)
from .errors import EkoDBError

T = TypeVar('T')

class EkoDBClient:
    def __init__(self, config: Union[ClientConfig, Dict[str, str]]):
        if isinstance(config, dict):
            config = ClientConfig(**config)
        self.base_url = config.base_url
        self.api_key = config.api_key
        self.token: Optional[str] = None
        self.session = requests.Session()
        self.session.headers.update({
            'Content-Type': 'application/json'
        })

    def _handle_error(self, error: Union[requests.RequestException, Exception]) -> None:
        if isinstance(error, requests.RequestException):
            status_code = error.response.status_code if error.response else None
            try:
                response_data = error.response.json() if error.response and error.response.content else None
            except ValueError:
                response_data = None
            
            raise EkoDBError(
                f"Request failed: {str(error)}",
                status_code=status_code,
                response=response_data
            )
        elif isinstance(error, pydantic.ValidationError):
            raise EkoDBError(
                f"Validation error: {str(error)}",
                status_code=422,
                response={"detail": str(error)}
            )
        else:
            raise EkoDBError(
                str(error),
                status_code=getattr(error, 'status_code', 500),
                response=getattr(error, 'response', None)
            )

    def _prepare_record(self, record: Dict[str, TypedValue[Any]]) -> Dict[str, Any]:
        return {k: v.dict() for k, v in record.items()}

    async def authenticate(self) -> None:
        try:
            if not self.api_key:
                raise EkoDBError("API key is required", status_code=400)

            response = requests.post(
                f"{self.base_url}/api/auth/token",
                json={"api_key": self.api_key}
            )
            
            if response.status_code != 200:
                raise EkoDBError(
                    f"Authentication failed: {response.json().get('message', '')}",
                    status_code=response.status_code,
                    response=response.json()
                )

            token_response = TokenResponse(**response.json())
            self.token = token_response.token
            self.session.headers.update({
                'Authorization': f'Bearer {self.token}'
            })
        except requests.RequestException as e:
            status_code = e.response.status_code if e.response else None
            try:
                response_data = e.response.json() if e.response and e.response.content else None
            except ValueError:
                response_data = None
            raise EkoDBError(
                f"Authentication failed: {str(e)}",
                status_code=status_code,
                response=response_data
            )
        except Exception as e:
            raise EkoDBError(
                str(e),
                status_code=getattr(e, 'status_code', None),
                response=getattr(e, 'response', None)
            )

    async def _ensure_authenticated(self) -> None:
        if not self.token:
            await self.authenticate()

    async def insert(self, collection: str, record: Dict[str, TypedValue[Any]]) -> str:
        try:
            await self._ensure_authenticated()
            response = self.session.post(
                f"{self.base_url}/api/insert/{collection}",
                json=self._prepare_record(record)
            )
            response.raise_for_status()
            return response.json()["id"]
        except Exception as e:
            self._handle_error(e)

    async def batch_insert(
        self,
        collection: str,
        records: List[Dict[str, TypedValue[Any]]]
    ) -> BatchInsertResponse:
        try:
            await self._ensure_authenticated()
            prepared_records = [self._prepare_record(record) for record in records]
            response = self.session.post(
                f"{self.base_url}/api/batch/insert/{collection}",
                json={"records": prepared_records}
            )
            response.raise_for_status()
            return BatchInsertResponse(**response.json())
        except Exception as e:
            self._handle_error(e)

    def _parse_typed_record(self, data: Dict[str, Any]) -> Dict[str, TypedValue[Any]]:
        return {k: TypedValue(**v) for k, v in data.items()}

    async def find(
        self,
        collection: str,
        options: Optional[FindOptions] = None
    ) -> List[Dict[str, TypedValue[Any]]]:
        try:
            await self._ensure_authenticated()
            params = options.dict(exclude_none=True) if options else {}
            response = self.session.get(
                f"{self.base_url}/api/find/{collection}",
                params=params
            )
            response.raise_for_status()
            return [self._parse_typed_record(record) for record in response.json()]
        except Exception as e:
            self._handle_error(e)

    async def find_by_id(
        self,
        collection: str,
        id: str
    ) -> Optional[Dict[str, TypedValue[Any]]]:
        try:
            await self._ensure_authenticated()
            response = self.session.get(
                f"{self.base_url}/api/find/{collection}/{id}"
            )
            if response.status_code == 404:
                return None
            response.raise_for_status()
            return self._parse_typed_record(response.json())
        except Exception as e:
            self._handle_error(e)

    async def update(
        self,
        collection: str,
        id: str,
        updates: Dict[str, TypedValue[Any]]
    ) -> bool:
        try:
            await self._ensure_authenticated()
            response = self.session.put(
                f"{self.base_url}/api/update/{collection}/{id}",
                json=self._prepare_record(updates)
            )
            response.raise_for_status()
            return response.json()["success"]
        except Exception as e:
            self._handle_error(e)

    async def batch_update(
        self,
        collection: str,
        updates: Dict[str, Dict[str, TypedValue[Any]]]
    ) -> BatchUpdateResponse:
        try:
            await self._ensure_authenticated()
            prepared_updates = {
                id: self._prepare_record(record)
                for id, record in updates.items()
            }
            response = self.session.put(
                f"{self.base_url}/api/batch/update/{collection}",
                json={"updates": prepared_updates}
            )
            response.raise_for_status()
            return BatchUpdateResponse(**response.json())
        except Exception as e:
            self._handle_error(e)

    async def delete(self, collection: str, id: str) -> bool:
        try:
            await self._ensure_authenticated()
            response = self.session.delete(
                f"{self.base_url}/api/delete/{collection}/{id}"
            )
            response.raise_for_status()
            return response.json()["success"]
        except Exception as e:
            self._handle_error(e)

    async def batch_delete(
        self,
        collection: str,
        ids: List[str]
    ) -> BatchDeleteResponse:
        try:
            await self._ensure_authenticated()
            response = self.session.delete(
                f"{self.base_url}/api/batch/delete/{collection}",
                json={"ids": ids}
            )
            response.raise_for_status()
            return BatchDeleteResponse(**response.json())
        except Exception as e:
            self._handle_error(e)

    async def get_schema(self, collection: str) -> CollectionSchema:
        try:
            await self._ensure_authenticated()
            response = self.session.get(
                f"{self.base_url}/api/schemas/{collection}"
            )
            response.raise_for_status()
            return CollectionSchema(**response.json())
        except Exception as e:
            self._handle_error(e)