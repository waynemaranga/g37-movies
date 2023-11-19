import pandas as pd
import re

class MovieDataCleaner:
    def __init__(self, data_file):
        self.data = pd.read_csv(data_file)

    def clean_data(self):
        '''
        This method cleans the data by: dropping rows with missing values in any essential column,
        cleaning the 'Rating' column, and converting columns to appropriate data types.
        '''
        
        self.data.dropna(subset=['User', 'Movie', 'Rating'], inplace=True) # Drop rows with missing values in any essential column by default
       
        self.clean_rating_column()  # Clean the 'Rating' column by extracting numeric values and handling invalid values

        # Convert columns to appropriate data types
        self.data['User'] = self.data['User'].astype(str) # converts from float to string
        self.data['Movie'] = self.data['Movie'].astype(str) # converts from float to string

        return self.data

    def clean_rating_column(self):
        '''
        This method cleans the 'Rating' column by extracting numeric values and handling invalid values.
        It does this by calling the extract_numeric_rating() method on each value in the 'Rating' column.
        '''
        # Use regular expressions to extract numeric ratings and handle invalid values
        self.data['Rating'] = self.data['Rating'].apply(lambda x: self.extract_numeric_rating(x))

    @staticmethod
    def extract_numeric_rating(value):
        '''
        This method extracts the first numeric value from a string.
        It does this by using a regular expression to extract the first numeric value from a string.
        This is important because the 'Rating' column contains values floating point values and NaN values.
        '''
        # Regular expression to extract the first numeric value
        match = re.search(r'\d+(\.\d+)?', str(value))
        if match:
            return float(match.group())
        else:
            # Return NaN if no valid numeric value found
            return None

    def write_to_csv(self, filename):
        '''
        This method writes the cleaned data to a CSV file.
        it does this by calling the to_csv() method on the data.
        '''
        self.data.to_csv(filename, index=False)
        print(f"Data written to {filename}")

    def write_to_json(self, filename):
        '''
        This method writes the cleaned data to a JSON file.
        it does this by calling the to_json() method on the data.
        '''
        self.data.to_json(filename, orient='records')
        print(f"Data written to {filename}")

# Instances
data_file = 'src/db/data.txt'
output_csv_file = 'src/data/cleaned_data.csv'
output_json_file = 'src/data/cleaned_data.json'

movie_data_cleaner = MovieDataCleaner(data_file)
cleaned_data = movie_data_cleaner.clean_data()

# Data Writers
movie_data_cleaner.write_to_csv(output_csv_file) # to CSV 
movie_data_cleaner.write_to_json(output_json_file) # to JSON
