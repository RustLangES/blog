import os, requests, uuid
import re

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

raw_date = 'DATE'
if not raw_date in os.environ:
    raise Exception('Please set/export the environment variable: {}'.format(raw_date))
raw_date = os.environ[raw_date]

if not os.path.isfile(raw_date + ".md"):
    raise Exception('The file not exists: {}'.format(raw_date))
raw_file = open(raw_date + ".md", 'r')

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

# generate ouput
meta_content = open(raw_date + "-this-week-in-rust.md", "r").read()
with open(raw_date + "-this-week-in-rust.md", 'w') as fh:
    content = response[0]["translations"][0]["text"]
    description = [line for line in content.split('\n') if line.startswith("La caja de esta semana es")]
    print(f"Match = {description}")
    description = description[0]
    finded = re.search(r'(\[(?P<caption>.*?)\])\((?P<image>.*?)(?P<description>\".*?\")?\)', description)
    if finded is None:
        print("No Encontrado")
        description = "Esta semana en Rust es un blog semanal sobre el lenguaje de programación Rust, sus comunidades y su ecosistema."
    else:
        print("Encontrado!!")
        finded = finded.groupdict()
        link_name = re.sub(r'\[.*\]\(.*\)', finded["caption"], description)
        print(f"To Replace: {meta_content}")
        new_content = meta_content.replace("Esta semana en Rust es un blog semanal sobre el lenguaje de programación Rust, sus comunidades y su ecosistema.", link_name)
        print(f"Replacement Result: {new_content}")
        content = new_content + '\n' + content
    fh.write(content)
