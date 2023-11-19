import pandas as pd

class MovieExtractor:
    def __init__(self, data_file):
        self.data = pd.read_csv(data_file)

    def extract_movies(self):
        unique_movies = self.data['Movie'].unique().tolist()
        return unique_movies

    def save_movies_to_json(self, filename):
        unique_movies = self.extract_movies()
        movies_dict = {'Movies': unique_movies}
        pd.DataFrame(movies_dict).to_json(filename, orient='records')
        print(f"Movies data written to {filename}")

# Example usage
data_file = 'src/data/cleaned_data.csv'
movies_output_json_file = 'src/data/movies.json'

movie_extractor = MovieExtractor(data_file)
movie_extractor.save_movies_to_json(movies_output_json_file)
