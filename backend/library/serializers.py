from library.models import Book
from rest_framework import serializers
from pathlib import Path
from django.conf import settings


class BookUploadSerializer(serializers.Serializer):
    book = serializers.FileField(write_only=True)
    title = serializers.CharField()

    def create(self, validated_data):
        uploaded_file = validated_data.pop("book")
        title = validated_data.pop("title")

        book = Book.objects.create(title=title)

        book_dir = Path(settings.LIBRARY_ROOT) / str(book.id)
        book_dir.mkdir(parents=True, exist_ok=True)
        file_path = book_dir / "book.epub"

        with open(file_path, "wb") as f:
            for chunk in uploaded_file.chunks():
                f.write(chunk)

        return book
