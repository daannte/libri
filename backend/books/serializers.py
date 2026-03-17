from pathlib import Path

from django.utils.text import slugify
from rest_framework import serializers

from books.models import Author, Book


class AuthorSerializer(serializers.ModelSerializer):
    class Meta:
        model = Author
        fields = ["id", "name", "bio"]


class BookSerializer(serializers.ModelSerializer):
    authors = AuthorSerializer(many=True)
    epub_file_url = serializers.SerializerMethodField()
    cover_image_url = serializers.SerializerMethodField()

    class Meta:
        model = Book
        fields = [
            "id",
            "title",
            "authors",
            "description",
            "epub_file_url",
            "cover_image_url",
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

    def get_epub_file_url(self, obj):
        return obj.epub_file.url if obj.epub_file else None

    def get_cover_image_url(self, obj):
        return obj.cover_image.url if obj.cover_image else None


class BookUploadSerializer(serializers.Serializer):
    book = serializers.FileField(write_only=True)

    def create(self, validated_data):
        uploaded_file = validated_data.pop("book")
        original_name = Path(uploaded_file.name).stem
        title = slugify(original_name) or "untitled"

        book = Book.objects.create(title=title)  # ty:ignore[unresolved-attribute]
        book.epub_file.save("book.epub", uploaded_file, save=True)

        return book
