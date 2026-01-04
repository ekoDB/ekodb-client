#!/usr/bin/env python3
"""
Test Runner for Python Examples
This script runs all Python examples as integration tests
"""

import asyncio
import os
import sys
import subprocess
import time
from pathlib import Path
import aiohttp
from dotenv import load_dotenv

# Load environment variables
env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")


# Colors for output
class Colors:
    GREEN = "\033[32m"
    RED = "\033[31m"
    YELLOW = "\033[33m"
    BLUE = "\033[34m"
    RESET = "\033[0m"


def log(message, color="RESET"):
    color_code = getattr(Colors, color, Colors.RESET)
    print(f"{color_code}{message}{Colors.RESET}")


async def check_server():
    """Check if server is running"""
    log("\n=== Checking Server Connection ===", "BLUE")

    async with aiohttp.ClientSession() as session:
        for i in range(30):
            try:
                async with session.get(
                    f"{BASE_URL}/api/health", timeout=aiohttp.ClientTimeout(total=1)
                ) as response:
                    if response.status == 200:
                        log("✓ Server is ready", "GREEN")
                        return True
            except Exception:
                sys.stdout.write(".")
                sys.stdout.flush()
                await asyncio.sleep(1)

    log("\n✗ Server not responding", "RED")
    return False


async def get_auth_token():
    """Get authentication token"""
    log("\n=== Getting Authentication Token ===", "BLUE")

    async with aiohttp.ClientSession() as session:
        try:
            async with session.post(
                f"{BASE_URL}/api/auth/token",
                json={"api_key": API_KEY},
                headers={"Content-Type": "application/json"},
            ) as response:
                if response.status != 200:
                    raise Exception(f"Auth failed: {response.status}")

                data = await response.json()
                log("✓ Authentication successful", "GREEN")
                return data["token"]
        except Exception as e:
            log(f"✗ Authentication failed: {e}", "RED")
            raise


def run_example(file_path, token):
    """Run a single example file"""
    log(f"\n=== Running {Path(file_path).name} ===", "BLUE")

    env = os.environ.copy()
    env.update(
        {
            "API_BASE_URL": BASE_URL,
            "WS_BASE_URL": os.getenv("WS_BASE_URL", "ws://localhost:8080"),
            "AUTH_TOKEN": token,
        }
    )

    try:
        result = subprocess.run(
            ["python3", file_path], env=env, capture_output=False, text=True, timeout=60
        )

        if result.returncode == 0:
            log(f"✓ {Path(file_path).name} completed successfully", "GREEN")
            return True
        else:
            log(f"✗ {Path(file_path).name} failed with code {result.returncode}", "RED")
            return False
    except subprocess.TimeoutExpired:
        log(f"✗ {Path(file_path).name} timed out", "RED")
        return False
    except Exception as e:
        log(f"✗ {Path(file_path).name} error: {e}", "RED")
        return False


async def main():
    """Main test runner"""
    log("\n╔════════════════════════════════════════╗", "BLUE")
    log("║   ekoDB Python Examples Test Suite    ║", "BLUE")
    log("╚════════════════════════════════════════╝", "BLUE")

    try:
        # Check server
        server_ready = await check_server()
        if not server_ready:
            sys.exit(1)

        # Get auth token
        token = await get_auth_token()

        # List of example files to run
        # Note: Direct API examples are in the main directory
        # TTL verification tests are in the ttl/ subdirectory
        base_dir = Path(__file__).parent
        examples = [
            base_dir / "simple_crud.py",
            base_dir / "simple_websocket.py",
            base_dir / "http_functions.py",
            base_dir / "batch_operations.py",
            base_dir / "kv_operations.py",
            base_dir / "collection_management.py",
            base_dir / "transactions.py",
            base_dir / "crud_scripts.py",
            base_dir / "ttl" / "document_ttl.py",
            base_dir / "ttl" / "websocket_ttl.py",
        ]

        log(f"\n=== Running {len(examples)} Examples ===", "YELLOW")

        passed = 0
        failed = 0

        for example in examples:
            if example.exists():
                if run_example(str(example), token):
                    passed += 1
                else:
                    failed += 1
                    log("Continuing with next example...", "YELLOW")

                # Wait between examples
                time.sleep(2)
            else:
                log(f"✗ Example not found: {example}", "RED")
                failed += 1

        # Summary
        log("\n╔════════════════════════════════════════╗", "BLUE")
        log("║           Test Summary                 ║", "BLUE")
        log("╚════════════════════════════════════════╝", "BLUE")
        log(f"Total: {len(examples)}", "BLUE")
        log(f"Passed: {passed}", "GREEN")
        log(f"Failed: {failed}", "RED" if failed > 0 else "GREEN")

        if failed > 0:
            sys.exit(1)

    except Exception as e:
        log(f"\n✗ Test suite failed: {e}", "RED")
        sys.exit(1)


if __name__ == "__main__":
    asyncio.run(main())
