import pandas as pd


def find_classes(courses: pd.DataFrame) -> pd.DataFrame:
    class_counts = courses.groupby("class")["student"].count().reset_index()
    return class_counts[class_counts["student"] >= 5][["class"]]
