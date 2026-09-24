from fastapi import APIRouter, Depends, Query, HTTPException
from sqlalchemy.orm import Session
from sqlalchemy import text
from app.database import get_db 
from app.processing.search import build_normal_query

router = APIRouter(tags=["Search Engine"])

# Lite MVP: direct MiniLM semantic search only. The full branch retains the
# power-search and hybrid-search implementation in the processing layer.
@router.post("/search/")
def normal_search_documents(
    query: str = Query(..., description="The exact search text from the user"),
    mode: str = Query("vector", description="Lite MVP uses vector semantic search"),
    top_k: int = Query(5, description="Number of results to return"),
    db: Session = Depends(get_db)
):
    """
    NORMAL SEARCH: Fast search using the exact text provided, bypassing LLM query expansion.
    """
    if mode != "vector":
        raise HTTPException(status_code=400, detail="Lite MVP supports vector search only")

    try:
        # Uses the new normal builder
        payload = build_normal_query(query, mode=mode, top_k=top_k)
        
        sql_string = text(payload["sql"][mode])
        params = payload["params"]
        raw_results = db.execute(sql_string, params).mappings().all()
        
        return {
            "status": "success",
            "search_type": "semantic",
            "query": query,
            "mode": mode,
            "results": [dict(row) for row in raw_results]
        }

    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Normal Search Error: {str(e)}")