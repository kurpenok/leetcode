import pandas as pd


def article_views(views: pd.DataFrame) -> pd.DataFrame:
    authors_viewed_own_articles = views[views["author_id"] == views["viewer_id"]]

    unique_authors = authors_viewed_own_articles["author_id"].unique()
    unique_authors = sorted(unique_authors)

    return pd.DataFrame({"id": unique_authors})
