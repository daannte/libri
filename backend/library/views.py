from rest_framework.response import Response
from library.models import Book
from library.serializers import BookUploadSerializer
from rest_framework.decorators import action
from rest_framework import viewsets, mixins, status


class BookViewSet(
    mixins.CreateModelMixin, mixins.ListModelMixin, viewsets.GenericViewSet
):
    def get_queryset(self):
        return Book.objects.all()

    def get_serializer_class(self):
        match self.action:
            case "create":
                return BookUploadSerializer
            case "list":
                # TODO: Change to BookSerializer
                return BookUploadSerializer
            case _:
                return super().get_serializer_class()

    def create(self, request, *args, **kwargs):
        serializer = self.get_serializer(data=request.data)
        serializer.is_valid(raise_exception=True)
        serializer.save()

        # TODO:return book info
        return Response(None, status=status.HTTP_201_CREATED)
