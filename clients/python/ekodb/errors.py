from typing import Optional, Any, Dict, Union

class EkoDBError(Exception):
    def __init__(
        self,
        message: str,
        status_code: Optional[int] = None,
        response: Optional[Union[Dict[str, Any], Any]] = None
    ):
        super().__init__(message)
        self.message = message
        self._status_code = status_code
        self._response = response
        self.name = "EkoDBError"

    @property
    def status_code(self) -> Optional[int]:
        if self._status_code is not None:
            return self._status_code
        if isinstance(self._response, dict):
            return self._response.get('status')
        return None

    @property
    def response(self) -> Optional[Union[Dict[str, Any], Any]]:
        return self._response

    def __str__(self) -> str:
        error_msg = self.message
        if self.status_code:
            error_msg += f" (Status: {self.status_code})"
        return error_msg