import pandas as pd

class MovieExtractor:
    '''
    This class extracts the unique movies from the data.
    It does this by calling the extract_movies() method on the data.
    the extract_movies() method returns a list of unique movies.
    '''
    def __init__(self, data_file):
        self.data = pd.read_csv(data_file)

    def extract_movies(self):
        '''
        This method extracts the unique movies from the data.
        It does this by calling the unique() and tolist() methods on the 'Movie' column.
        '''
        unique_movies = self.data['Movie'].unique().tolist()
        return unique_movies

    def save_movies_to_json(self, filename):
        '''
        This method saves the unique movies to a JSON file.
        '''
        unique_movies = self.extract_movies()
        movies_dict = {'Movies': unique_movies}
        pd.DataFrame(movies_dict).to_json(filename, orient='records')
        print(f"Movies data written to {filename}")


data_file = 'src/data/cleaned_data.csv' # Path to the cleaned data
movies_output_json_file = 'src/data/movies.json' # Path to the output JSON file

movie_extractor = MovieExtractor(data_file) # Instance of the MovieExtractor class
movie_extractor.save_movies_to_json(movies_output_json_file) # Save the unique movies to a JSON file
