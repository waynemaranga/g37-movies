import json
from collections import Counter
import os

class MovieDataSummary:
    def __init__(self, cleaned_data):
        self.cleaned_data = cleaned_data

    def top_watched_movies(self, n=5):
        movie_counts = Counter(movie['Movie'] for movie in self.cleaned_data)
        return movie_counts.most_common(n)

    def highest_rated_movies(self, n=5):
        movie_ratings = {movie['Movie']: [] for movie in self.cleaned_data if movie['Rating'] is not None}
        for movie in self.cleaned_data:
            if movie['Rating'] is not None:
                movie_ratings[movie['Movie']].append(movie['Rating'])

        average_ratings = {movie: sum(ratings) / len(ratings) for movie, ratings in movie_ratings.items()}
        return sorted(average_ratings.items(), key=lambda x: x[1], reverse=True)[:n]

    def summarize_data(self):
        return {
            'top_watched_movies': self.top_watched_movies(),
            'highest_rated_movies': self.highest_rated_movies(),
        }

    def save_summary_to_json(self, folder_path='src/data'):
        summary = self.summarize_data()
        
        # Create the output folder if it doesn't exist
        os.makedirs(folder_path, exist_ok=True)
        
        file_path = os.path.join(folder_path, 'summary.json')
        with open(file_path, 'w') as json_file:
            json.dump(summary, json_file, indent=2)

    def save_summary_to_text(self, folder_path='src/data'):
        summary = self.summarize_data()
        
        # Create the output folder if it doesn't exist
        os.makedirs(folder_path, exist_ok=True)
        
        file_path = os.path.join(folder_path, 'summary.txt')
        with open(file_path, 'w') as text_file:
            text_file.write("\nTop Watched Movies:\n")
            for movie, count in summary['top_watched_movies']:
                text_file.write(f"{movie}: {count} views\n")

            text_file.write("\nHighest Rated Movies:\n")
            for movie, rating in summary['highest_rated_movies']:
                text_file.write(f"{movie}: Average Rating - {rating:.2f}\n")

if __name__ == '__main__':
    # Specify the path to the cleaned_data.json file
    cleaned_data_path = 'src/data/cleaned_data.json'

    # Load cleaned data from the file
    with open(cleaned_data_path, 'r') as json_file:
        cleaned_data = json.load(json_file)

    # Create an instance of MovieDataSummary
    movie_summary = MovieDataSummary(cleaned_data)

    # Save the summary to a JSON file in the 'output' folder
    movie_summary.save_summary_to_json()

    # Save the summary to a text file in the 'output' folder
    movie_summary.save_summary_to_text()
