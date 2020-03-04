This is a katalon studio project, it should be open in katalon studio
The Object repository folder contains the individual tests for each method GET, PUT, UPDATE, DELETE)
The Test cases folder contains a test case that strings all of the methods together 
The Test Suite folder contains a test suite that makes all the requests in the Test cases folder and iterates through the data found in the Data files folder. There is a Suite that is supposed to pass and one destined to fail.
The Files folder contains two Excel files, one for data that should pass and another for data that should fail. There is also a .txt file with more uncorrupted data.

Notes:
The put and the delete method are not responding with expected status codes
The put method's response is not in the expected format