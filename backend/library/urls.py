from django.urls import path, include
from library import views
from rest_framework.routers import DefaultRouter

router = DefaultRouter()
router.register("", views.BookViewSet, basename="book")

urlpatterns = [path("", include(router.urls))]
