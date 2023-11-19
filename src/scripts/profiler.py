import pandas as pd

class UserProfileGenerator:
    def __init__(self, data_file):
        self.data = pd.read_csv(data_file)

    def generate_profiles(self):
        profiles = {}
        for _, row in self.data.iterrows():
            user = row['User']
            movie = row['Movie']
            rating = row['Rating']

            if user not in profiles:
                profiles[user] = {'Movies': {}}

            profiles[user]['Movies'][movie] = rating

        return profiles

    def save_profiles_to_json(self, filename):
        profiles = self.generate_profiles()
        pd.DataFrame(profiles).T.to_json(filename, orient='index')
        print(f"Profiles data written to {filename}")

# Example usage
data_file = 'src/data/cleaned_data.csv'
profiles_output_json_file = 'src/data/profiles.json'

profile_generator = UserProfileGenerator(data_file)
profile_generator.save_profiles_to_json(profiles_output_json_file)
