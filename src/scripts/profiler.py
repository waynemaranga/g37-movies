import pandas as pd

class UserProfileGenerator:
    '''
    This class generates user profiles from the data.
    It does this by calling the generate_profiles() method on the data.
    The generate_profiles() method returns a dictionary of user profiles.
    '''
    def __init__(self, data_file):
        '''
        This method initializes the class by reading the data from a CSV file.
        The data is stored in the 'data' attribute. The data is a Pandas DataFrame.
        '''
        self.data = pd.read_csv(data_file)

    def generate_profiles(self):
        '''
        this method generates user profiles from the data.
        '''
        profiles = {}
        for _, row in self.data.iterrows():
            user = row['User']
            movie = row['Movie']
            rating = row['Rating']

            if user not in profiles:
                profiles[user] = {'Movies': {}}

            profiles[user]['Movies'][movie] = rating

        return profiles

    def save_profiles_to_json(self, filename): # pylint: disable=invalid-name
        profiles = self.generate_profiles() # Generate user profiles
        pd.DataFrame(profiles).T.to_json(filename, orient='index')
        print(f"Profiles data written to {filename}")


data_file = 'src/data/cleaned_data.csv' # Path to the cleaned data
profiles_output_json_file = 'src/data/profiles.json' # Path to the output JSON file

profile_generator = UserProfileGenerator(data_file)  # Instance of the UserProfileGenerator class
profile_generator.save_profiles_to_json(profiles_output_json_file) # to CSV 
