import json
import random
import numpy as np
from sklearn.metrics.pairwise import cosine_similarity

class SimpleMovieRecommender:
    def __init__(self, movies_data):
        self.movies_data = movies_data
        self.movie_names = list(self.movies_data.keys())
        self.user_movie_matrix = self.create_user_movie_matrix()

    def create_user_movie_matrix(self):
        # Create a user-movie matrix
        unique_movies = set(movie for movies in self.movies_data.values() for movie in movies)
        user_movie_matrix = np.zeros((len(self.movie_names), len(unique_movies)), dtype=int)
        
        for i, movie in enumerate(self.movie_names):
            for j, similar_movie in enumerate(unique_movies):
                if similar_movie in self.movies_data[movie]:
                    user_movie_matrix[i, j] = 1
        
        return user_movie_matrix

    def recommend_movies(self, movie_title, n=5):
        # Calculate cosine similarity between the target movie and all other movies
        target_movie_index = self.movie_names.index(movie_title)
        target_movie_vector = self.user_movie_matrix[target_movie_index].reshape(1, -1)
        similarity_scores = cosine_similarity(self.user_movie_matrix, target_movie_vector)

        # Get indices of movies with the highest similarity
        similar_movies_indices = np.argsort(similarity_scores.flatten())[:-n-1:-1]

        # Get movie titles
        recommended_movies = [self.movie_names[i] for i in similar_movies_indices]

        # Exclude the target movie from recommendations
        recommended_movies = [movie for movie in recommended_movies if movie != movie_title]

        return recommended_movies[:n]

    def get_movie_list(self):
        # Display a numbered list of movies
        print("Movie List:")
        for i, movie in enumerate(self.movie_names, start=1):
            print(f"{i}. {movie}")

    def get_movie_from_user_input(self):
        # Get user input for choosing a movie
        while True:
            try:
                self.get_movie_list()
                choice = int(input("Enter the number of the movie you want recommendations for: "))
                if 1 <= choice <= len(self.movie_names):
                    return self.movie_names[choice - 1]
                else:
                    print("Invalid choice. Please enter a valid number.")
            except ValueError:
                print("Invalid input. Please enter a number.")

# Specify the file path for the movies_similar.json file
movies_similar_file_path = 'src/db/movies_similar.json'

# Read data from the file
with open(movies_similar_file_path, 'r') as json_file:
    movies_data = json.load(json_file)

# Example usage
simple_movie_recommender = SimpleMovieRecommender(movies_data)

# Get a movie from user input
user_selected_movie = simple_movie_recommender.get_movie_from_user_input()

# Get recommendations for the user-selected movie
recommended_movies = simple_movie_recommender.recommend_movies(user_selected_movie)

print(f"Top 5 recommended movies for '{user_selected_movie}':")
for movie in recommended_movies:
    print(movie)
