import os, requests, uuid, json

key_var_name = 'API_KEY'
if not key_var_name in os.environ:
    raise Exception('Please set/export the environment variable: {}'.format(key_var_name))
resource_key = os.environ[key_var_name]

region_var_name = 'REGION'
if not region_var_name in os.environ:
    raise Exception('Please set/export the environment variable: {}'.format(region_var_name))
region = os.environ[region_var_name]

endpoint_var_name = 'API_ENDPOINT'
if not endpoint_var_name in os.environ:
    raise Exception('Please set/export the environment variable: {}'.format(endpoint_var_name))
endpoint = os.environ[endpoint_var_name]

path = '/translate?api-version=3.0'
params = '&from=en&to=es'
constructed_url = endpoint + path + params

raw_file_name = 'DATE'
if not raw_file_name in os.environ:
    raise Exception('Please set/export the environment variable: {}'.format(endpoint_var_name))
raw_file_name = os.environ[raw_file_name]

if not os.path.isfile(raw_file_name + ".md"):
    raise Exception('The file not exists: {}'.format(raw_file_name))
raw_file = open(raw_file_name + ".md", 'r')

headers = {
    'Ocp-Apim-Subscription-Key': resource_key,
    'Ocp-Apim-Subscription-Region': region,
    'Content-type': 'application/json',
    'X-ClientTraceId': str(uuid.uuid4())
}

# You can pass more than one object in body.
body = [{
    'text' : raw_file.read()
}]
request = requests.post(constructed_url, headers=headers, json=body)
response = request.json()

raw_file.close()

print(json.dumps(response, indent=4))

# generate ouput
with open(raw_file_name + "-this-week-in-rust.md", 'a') as fh:
    print(response[0]["translations"][0]["text"], file = fh)
