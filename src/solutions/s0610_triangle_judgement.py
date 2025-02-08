import pandas as pd


def triangle_judgement(df: pd.DataFrame) -> pd.DataFrame:
    bf1 = df[(df.x >= df.y + df.z) | (df.y >= df.x + df.z) | (df.z >= df.y + df.x)]
    bf1["triangle"] = "No"
    bf2 = df[(df.x < df.y + df.z) & (df.y < df.x + df.z) & (df.z < df.y + df.x)]
    bf2["triangle"] = "Yes"
    return pd.concat([bf1, bf2])
