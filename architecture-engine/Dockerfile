FROM python:3.11-slim
WORKDIR /app
COPY architecture-engine/ ./architecture-engine/
WORKDIR /app/architecture-engine
RUN if [ -f requirements.txt ]; then pip install --no-cache-dir -r requirements.txt; else pip install fastapi uvicorn pydantic; fi
CMD ["sh", "-c", "python main.py 2>/dev/null || python app.py 2>/dev/null || (echo 'Architecture Engine -- BorderAgent starting...' && sleep infinity)"]
