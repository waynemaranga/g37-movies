import pandas as pd

class UserExtractor:
    def __init__(self, data_file):
        self.data = pd.read_csv(data_file)

    def extract_users(self):
        unique_users = self.data['User'].unique().tolist()
        return unique_users

    def save_users_to_json(self, filename):
        unique_users = self.extract_users()
        users_dict = {'Users': unique_users}
        pd.DataFrame(users_dict).to_json(filename, orient='records')
        print(f"\nUsers data written to {filename}")

# Example usage
data_file = 'src/data/cleaned_data.csv'
users_output_json_file = 'src/data/users.json'

user_extractor = UserExtractor(data_file)
user_extractor.save_users_to_json(users_output_json_file)
