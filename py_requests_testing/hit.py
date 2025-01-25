
import requests
import json

def test_echo_endpoint(): 

    #      http://127.0.0.1:8080/echo_input_data
    url = "http://127.0.0.1:8080/echo_input_data"
    
    # Test data
    data = "Hello, World!"
    
    # Headers
    headers = {
        'Content-Type': 'text/plain'
    }
    
    try:
        # Make POST request
        response = requests.post(url, data=data, headers=headers)
        
        # Print detailed information
        print(f"Status Code: {response.status_code}")
        print(f"Headers: {dict(response.headers)}")
        print(f"Response Body: {response.text}")
        
        # Optional: try to parse as JSON if that's what we expect
        try:
            json_response = response.json()
            print(f"JSON Response: {json_response}")
        except:
            print("Response was not JSON")
            
    except requests.exceptions.RequestException as e:
        print(f"Request failed: {e}")

if __name__ == "__main__":
    test_echo_endpoint()
