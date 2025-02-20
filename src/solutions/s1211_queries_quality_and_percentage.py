import pandas as pd


def queries_stats(df: pd.DataFrame) -> pd.DataFrame:
    df = df.assign(
        quality=df.rating / df.position + 1e-10,
        poor_query_percentage=(df.rating < 3).astype(int) * 100,
    )
    return (
        df.groupby("query_name", as_index=False)[["quality", "poor_query_percentage"]]
        .mean()
        .round(2)
    )
