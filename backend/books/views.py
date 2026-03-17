from books.models import Book
from books.serializers import BookSerializer, BookCreateSerializer
from rest_framework import viewsets


class BookViewSet(viewsets.ModelViewSet):
    def get_queryset(self):
        return Book.objects.all()  # ty:ignore[unresolved-attribute]

    def get_serializer_class(self):
        if self.action == "create":
            return BookCreateSerializer
        return BookSerializer
