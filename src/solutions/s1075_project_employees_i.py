import pandas as pd


def project_employees_i(project: pd.DataFrame, employee: pd.DataFrame) -> pd.DataFrame:
    df = pd.merge(project, employee, how="left", on="employee_id")

    result = (
        df.groupby("project_id")["experience_years"]
        .mean()
        .round(2)
        .rename("average_years")
        .reset_index()
    )

    return result
