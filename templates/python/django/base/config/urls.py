from django.http import JsonResponse
from django.urls import path


def health(_request):
    return JsonResponse({"ok": True, "service": "{{name}}"})


urlpatterns = [
    path("health/", health),
]
