from http import HTTPMethod
from rest_framework.response import Response
from rest_framework.decorators import action
from books.models import Book
from books.serializers import BookSerializer, BookCreateSerializer, EnrichBookSerializer
from rest_framework import viewsets, status


class BookViewSet(viewsets.ModelViewSet):
    def get_queryset(self):
        return Book.objects.all()  # ty:ignore[unresolved-attribute]

    def get_serializer_class(self):
        if self.action == "create":
            return BookCreateSerializer
        elif self.action == "enrich":
            return super().get_serializer_class()
        return BookSerializer

    @action(
        detail=True,
        methods=[HTTPMethod.POST],
        url_path="enrich",
        serializer_class=EnrichBookSerializer,
    )
    def enrich(self, request, pk=None):
        book = self.get_object()
        serializer = self.get_serializer(data=request.data)
        serializer.is_valid(raise_exception=True)
        book = serializer.update(book, serializer.validated_data)

        output = BookSerializer(book)

        return Response(output.data, status=status.HTTP_200_OK)
