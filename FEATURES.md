# FEATURES

## `main.rs`
Entry point of the Rust application, responsible for executing Python scripts
that handle data cleaning, summarization, and recommendation. Includes an ASCII
art display with color to enhance user experience. Coordinates seamless
integration between Rust and Python for a robust, cross-language solution.

## `python_runner.rs`
Rust module defining functions for executing Python scripts with a loading bar,
enhancing the visual presentation of data processing tasks. Utilizes the termion
crate for efficient terminal formatting. Contributes to a smoother user
interaction by providing visual feedback during Python script execution.

## `summary.rs`
Rust module providing functions to calculate top-watched and highest-rated
movies, contributing to the data summarization process in the application.
Integrates seamlessly with Python scripts for a comprehensive summarization
solution. Enhances the Rust application's capabilities by incorporating
statistical insights.

## `clean.py`
Python script featuring the `MovieDataCleaner` class, which systematically
cleans movie data, dropping missing values, cleaning the 'Rating' column, and
converting data types. The class writes the cleaned data to CSV and JSON files,
ensuring data integrity. Promotes reproducibility by encapsulating data cleaning
steps within a reusable class.

## `movies.py`
Python script containing the `MovieExtractor` class, responsible for
extracting unique movie names from the cleaned dataset and saving them to a JSON
file. This class enhances data exploration capabilities by providing a distinct
list of movies. Facilitates further analysis by offering a straightforward
extraction method for movie names.

## `profiler.py`
Python script housing the `UserProfileGenerator` class, which generates user
profiles from cleaned data. The class saves these profiles to a JSON file,
offering insights into user preferences and enhancing the personalization of
recommendations. Provides valuable data for targeted marketing and user-focused
content delivery.

## `recommender_input.py`
Python script implementing a simple movie recommender system with the
`SimpleMovieRecommender` class. Users can input a movie choice to receive
recommendations based on cosine similarity, promoting an interactive
experience. Encourages user engagement by providing personalized
recommendations tailored to individual movie preferences.

## `summary.py`
Python script with the `MovieDataSummary` class, responsible for summarizing
cleaned movie data and saving the summary to JSON and text files. This class
provides a concise overview of essential statistics, aiding in data
interpretation. Enhances data reporting by generating easily interpretable
summaries for further analysis.

## `users.py`
Python script featuring the `UserExtractor` class, which extracts unique user
identifiers from the cleaned dataset and saves them to a JSON file. This class
enhances the understanding of the user base within the movie dataset.
Contributes to user profiling and facilitates targeted content delivery based
on user-specific patterns.

## Python Packages Used
- `pandas`: Used for data manipulation and analysis in Python scripts.
- `json`: Used for JSON file handling in Python scripts.
- `numpy`: Used for numerical operations in the recommender system.
- `scikit-learn`: Used for cosine similarity calculations in the recommender
  system.

## Rust Crates Used
- `termion`: Used for terminal formatting and color in the Rust application.

## Summary
The project combines Rust and Python to create a movie data processing
application. Rust manages the overall execution, while Python scripts handle
specific tasks like data cleaning, summarization, and recommendation. The use
of Python packages such as `pandas`, `json`, and `scikit-learn` enriches data
processing capabilities, and the integration with the `termion` crate enhances
the user interface. The combination of both languages results in a versatile
and efficient solution for managing and analyzing movie data.
