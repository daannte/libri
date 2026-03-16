from pathlib import Path

from django.conf import settings
from django.utils.text import slugify
from rest_framework import serializers

from library.models import Author, Book


class AuthorSerializer(serializers.ModelSerializer):
    class Meta:
        model = Author
        fields = ["id", "name", "bio"]


class BookSerializer(serializers.ModelSerializer):
    authors = AuthorSerializer(many=True)

    class Meta:
        model = Book
        fields = [
            "id",
            "title",
            "authors",
            "description",
            "asin",
            "isbn",
            "isbn13",
            "pages",
            "publication_time",
            "publisher",
            "language",
            "currently_reading",
            "read",
            "type",
            "created_at",
        ]


class BookUploadSerializer(serializers.Serializer):
    book = serializers.FileField(write_only=True)

    def create(self, validated_data):
        uploaded_file = validated_data.pop("book")

        original_name = Path(uploaded_file.name).stem  # remove extension
        title = slugify(original_name) or "untitled"

        book = Book.objects.create(title=title)  # ty:ignore[unresolved-attribute]

        book_dir = Path(settings.LIBRARY_ROOT) / str(book.id)
        book_dir.mkdir(parents=True, exist_ok=True)

        file_path = book_dir / "book.epub"
        with open(file_path, "wb") as f:
            for chunk in uploaded_file.chunks():
                f.write(chunk)

        return book
