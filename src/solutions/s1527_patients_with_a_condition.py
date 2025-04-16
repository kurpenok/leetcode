import pandas as pd


def find_patients(patients: pd.DataFrame) -> pd.DataFrame:
    return patients.loc[patients["conditions"].str.contains(r"(^| )DIAB1")]
