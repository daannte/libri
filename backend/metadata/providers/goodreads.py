from datetime import datetime
import json
from bs4 import BeautifulSoup
import requests
from metadata.providers.base import MetadataProvider
from metadata.serializers import MetadataBookSerializer


class GoodreadsProvider(MetadataProvider):
    name = "Goodreads"

    URL: str = "https://www.goodreads.com/book/show/"

    def search(self, title: str) -> dict | None:
        r = requests.get(f"{self.URL}{title}")
        r.raise_for_status()

        soup = BeautifulSoup(r.text, "html.parser")
        script_tag = soup.find("script", id="__NEXT_DATA__")
        if not script_tag or not script_tag.string:
            return None

        data = json.loads(script_tag.string)
        apollo_state = data["props"]["pageProps"]["apolloState"]
        book_info = self._get_book_info(apollo_state)
        if not book_info:
            return None

        book_dict = {
            "title": book_info.get("title", ""),
            "authors": self._extract_authors(apollo_state),
            "description": book_info.get("description", ""),
            "cover_image_url": book_info.get("imageUrl"),
            "asin": None,
            "isbn": None,
            "isbn13": None,
            "pages": None,
            "publication_time": None,
            "publisher": None,
            "language": None,
        }

        details = book_info.get("details", {})
        if details:
            ts = details.get("publicationTime")
            publication_time = datetime.fromtimestamp(ts / 1000) if ts else None
            lang = details.get("language") or {}

            book_dict.update(
                asin=details.get("asin"),
                isbn=details.get("isbn"),
                isbn13=details.get("isbn13"),
                pages=details.get("numPages"),
                publication_time=publication_time,
                publisher=details.get("publisher"),
                language=lang.get("name"),
            )

        serializer = MetadataBookSerializer(book_dict)
        return serializer.data

    def _get_book_info(self, apollo_state: dict[str, dict]) -> dict | None:
        for key, val in apollo_state.items():
            if "Book:kca" in key:
                return val
        return None

    def _get_author_keys(self, apollo_state: dict[str, dict]) -> list[str]:
        return [key for key in apollo_state if "Contributor:kca" in key]

    def _extract_authors(self, apollo_state: dict[str, dict]) -> list[dict]:
        authors = []
        for key in self._get_author_keys(apollo_state):
            author_details = apollo_state.get(key)
            if not author_details or author_details.get("name") is None:
                continue
            authors.append(
                {
                    "name": author_details.get("name", ""),
                    "bio": author_details.get("description") or "",
                }
            )
        return authors
