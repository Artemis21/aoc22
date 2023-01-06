from __future__ import annotations

import json
import pathlib
import re
import time
import webbrowser
from typing import Literal, TypedDict, Any

import bs4
import rich
import requests

YEAR = 2022
BASE_PATH = pathlib.Path(__file__).parent
TOKEN_FILE = BASE_PATH / ".token"  # Advent of Code session cookie
INPUTS_FILE = BASE_PATH / "inputs.json"
SUBMISSIONS_FILE = BASE_PATH / "submissions.json"
URL = f"https://adventofcode.com/{YEAR}/day/{{day}}{{endpoint}}"

with open(TOKEN_FILE) as f:
    AUTH_COOKIES = {"session": f.read().strip()}


def load_json(path: pathlib.Path) -> dict[str, Any]:
    """Load a JSON file."""
    try:
        with open(path) as f:
            return json.load(f)
    except FileNotFoundError:
        return {}


def request(
    method: Literal["GET", "POST"],
    day: int,
    endpoint: str,
    data: dict[str, Any] | None = None
) -> str:
    """Make a request to a given endpoint for a given day."""
    response = requests.request(
        method=method,
        url=URL.format(day=day, endpoint=endpoint),
        cookies=AUTH_COOKIES,
        data=data,
    )
    if not response.ok:
        raise ValueError(f"Bad server response code: {response.text}.")
    return response.text


def get_input(day: int) -> str:
    """Get the raw input for a given day.

    Inputs are cached in the INPUTS_FILE.
    """
    day_key = str(day)
    # Check if the input is already cached.
    inputs = load_json(INPUTS_FILE)
    if day_key in inputs:
        return inputs[day_key]
    # Get the input from the server.
    input = request("GET", day, "/input").strip("\n")
    # Add the new input to the cache.
    inputs[day_key] = input
    with open(INPUTS_FILE, "w") as f:
        json.dump(inputs, f, indent=4)
    return input


class SubmissionResponseMessage(TypedDict):
    """A message previously displayed for a response."""

    message: str
    colour: str


class SubmissionResponse:
    """A response from a submission request."""

    _cache: dict[str, dict[str, SubmissionResponseMessage]] = {}

    @classmethod
    def _load_cache(cls):
        """Load the cache from the cache file."""
        cls._cache = load_json(SUBMISSIONS_FILE)

    @classmethod
    def _dump_cache(cls):
        """Store the cache in the cache file."""
        with open(SUBMISSIONS_FILE, "w") as f:
            json.dump(cls._cache, f, indent=4)

    @classmethod
    def submit(
        cls,
        day: int,
        solution: str | int | None,
        part: int,
    ) -> SubmissionResponse:
        """Submit a solution for a given day."""
        cls._load_cache()
        cache_key = f"{day}-{part}"
        if cache_key not in cls._cache:
            cls._cache[cache_key] = {}
        # Calculate the solution.
        if solution is None:
            return cls("yellow", "Solving function returned None.")
        solution = str(solution)
        # Make sure we haven't already tried this solution.
        if response := cls._cache[cache_key].get(solution):
            return cls(response["colour"], "Already submitted: {message}".format(**response))
        while True:
            rich.print(f"Submitting {solution} as solution to day {day} part {part}:")
            response = request("POST", day, "/answer", {"level": str(part), "answer": solution})
            message = cls.parse(response)
            if isinstance(message, SubmissionRatelimitResponse):
                message()  # Retry the timeout (calling message() will wait the cooldown).
                continue
            break
        # Add the response to the cache.
        cls._cache[cache_key][solution] = {
            "colour": message.colour,
            "message": message.message,
        }
        cls._dump_cache()
        return message

    @classmethod
    def parse(cls, response: str) -> SubmissionResponse:
        """Parse a server response to a submission."""
        soup = bs4.BeautifulSoup(response, "html.parser")
        if not soup.article:
            raise ValueError(f"Failed to parse server response: {response}.")
        message = soup.article.text.replace("\n", "").strip()
        if message.startswith("You gave"):
            wait_re = r"You have (?:(\d+)m )?(\d+)s left to wait."
            (minutes, seconds), = re.findall(wait_re, message)
            return SubmissionRatelimitResponse(message, minutes, seconds)
        if message.startswith("That's the"):
            return cls("green", message, new_success=True)
        elif message.startswith("You don't"):
            return cls("yellow", message)
        elif message.startswith("That's not"):
            return cls("red", message)
        else:
            raise ValueError(f"Failed to parse server message: {message}.")

    def __init__(self, colour: str, message: str, new_success: bool = False):
        """Store the message for the response."""
        self.colour = colour
        self.message = message
        self.new_success = new_success

    def __call__(self):
        """Print the message."""
        rich.print(f"[bold {self.colour}]{self.message}[/bold {self.colour}]")


class SubmissionRatelimitResponse(SubmissionResponse):
    """A response indicating we should retry after a give time period."""

    def __init__(self, message: str, minutes: str, seconds: str):
        """Store the data for the response."""
        self.minutes = int(minutes or "0")
        self.seconds = int(seconds)
        super().__init__("yellow", message)

    def __call__(self):
        """Wait the given time after printing the message."""
        super().__call__()
        rich.print(f"Waiting {self.minutes}m {self.seconds}s seconds to retry...")
        time.sleep(self.minutes * 60 + self.seconds)


class Solution:
    """A base class for solutions to a day's problems."""

    day: int

    def __init__(self, raw: str):
        """Parse and store the raw data."""

    def part_1(self) -> str | int | None:
        """Calculate the answer for part 1."""
        raise NotImplementedError

    def part_2(self) -> str | int | None:
        """Calculate the answer for part 2."""
        raise NotImplementedError

    def submit_1(self, logs: bool = False) -> SubmissionResponse:
        """Submit the solution for part 1."""
        rich.print(f"Submitting result to day {self.day} part 1...")
        response = SubmissionResponse.submit(self.day, self.part_1(), 1)
        response()
        return response

    def submit_2(self, logs: bool = False) -> SubmissionResponse:
        """Submit the solution for part 2."""
        rich.print(f"Submitting result to day {self.day} part 2...")
        response = SubmissionResponse.submit(self.day, self.part_2(), 2)
        response()
        return response

    @classmethod
    def get_instance(cls) -> "Solution":
        """Return an instance of the solution."""
        return cls(get_input(cls.day))

    @classmethod
    def submit(cls, logs: bool = False):
        """Submit the answer to both parts."""
        solution_inst = cls.get_instance()
        response_1 = solution_inst.submit_1()
        if response_1.new_success:
            part_2_url = URL.format(day=cls.day, endpoint="#part2")
            webbrowser.open(part_2_url)
        solution_inst.submit_2()
