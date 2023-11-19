import json
import random
import numpy as np
from sklearn.metrics.pairwise import cosine_similarity

class SimpleMovieRecommender:
    '''
    this class implements a simple movie recommender system.It takes a dictionary of
    movie titles and a list of similar movies as input. It then creates a user-movie matrix
    from the data and uses cosine similarity to recommend movies. It also provides a method
    to get a movie from user input.
    '''
    def __init__(self, movies_data):
        '''
        This method initializes the class by taking a dictionary of movie titles and a list of
        similar movies as input. It then creates a user-movie matrix from the data.
        '''
        self.movies_data = movies_data
        self.movie_names = list(self.movies_data.keys())
        self.user_movie_matrix = self.create_user_movie_matrix()

    def create_user_movie_matrix(self):
        # Create a user-movie matrix
        unique_movies = set(movie for movies in self.movies_data.values() for movie in movies)
        user_movie_matrix = np.zeros((len(self.movie_names), len(unique_movies)), dtype=int)
        
        for i, movie in enumerate(self.movie_names): # pylint: disable=invalid-name
            for j, similar_movie in enumerate(unique_movies):
                if similar_movie in self.movies_data[movie]:
                    user_movie_matrix[i, j] = 1
        
        return user_movie_matrix

    def recommend_movies(self, movie_title, n=5):
        # calculate cosine similarity between the target movie and all other movies
        # in the user-movie matrix. Then return the top n movies with the highest similarity score.
        target_movie_index = self.movie_names.index(movie_title) # pylint: disable=invalid-name
        target_movie_vector = self.user_movie_matrix[target_movie_index].reshape(1, -1) # pylint: disable=invalid-name
        similarity_scores = cosine_similarity(self.user_movie_matrix, target_movie_vector) # pylint: disable=invalid-name, no-member

        # get indices of movies with the highest similarity
        similar_movies_indices = np.argsort(similarity_scores.flatten())[:-n-1:-1] # pylint: disable=invalid-name, no-member

        # Get movie titles
        recommended_movies = [self.movie_names[i] for i in similar_movies_indices]

        # Exclude the target movie from recommendations
        recommended_movies = [movie for movie in recommended_movies if movie != movie_title]

        return recommended_movies[:n]

    def get_movie_list(self):
        # Display a numbered list of movies
        print("\nMovie List:")
        for i, movie in enumerate(self.movie_names, start=1):
            print(f"{i}. {movie}")

    def get_movie_from_user_input(self):
        '''
        This method gets a movie from user input. It first displays a numbered list of movies.
        It then asks the user to enter the number of the movie they want recommendations for.
        It then returns the movie name. It also validates the user input.
        '''
        while True:
            try:
                self.get_movie_list()
                choice = int(input("Enter the number of the movie you want recommendations for: "))
                if 1 <= choice <= len(self.movie_names): # pylint: disable=invalid-name
                    return self.movie_names[choice - 1]
                    # return the movie name at the index specified by the user
                else:
                    print("Invalid choice. Please enter a valid number.")
            except ValueError: 
                print("Invalid input. Please enter a number.")
                # ValidationError is raised if the user enters a non-integer value


movies_similar_file_path = 'src/db/movies_similar.json' # Path to the movies_similar.json file

# Read data from the file
with open(movies_similar_file_path, 'r') as json_file:
    movies_data = json.load(json_file)

simple_movie_recommender = SimpleMovieRecommender(movies_data) # Instance of the SimpleMovieRecommender class
user_selected_movie = simple_movie_recommender.get_movie_from_user_input() # Get the movie from user input
recommended_movies = simple_movie_recommender.recommend_movies(user_selected_movie) 

print(f"\n\nTop 5 recommended movies for '{user_selected_movie}':")
for movie in recommended_movies:
    print(movie)
    # print the recommended movies
