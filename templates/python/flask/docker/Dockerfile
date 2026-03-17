FROM python:3.12-slim
WORKDIR /workspace
COPY . .
RUN pip install -e .
EXPOSE 5000
CMD ["python", "app/main.py"]
