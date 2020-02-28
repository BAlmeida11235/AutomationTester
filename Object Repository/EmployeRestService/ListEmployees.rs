<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ListEmployees</name>
   <tag></tag>
   <elementGuidId>053d3fe7-3c18-40f0-a95b-39f0981d284b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://dummy.restapiexample.com/api/v1/employees</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable
import java.util.logging.Logger

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

//Verify status code
WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

//Check response status
WS.verifyElementPropertyValue(response, 'status', &quot;success&quot;)


//Make sure List of employees is correct
for(i = GlobalVariable.employeeList.data[0].id;
	 i &lt; GlobalVariable.employeeList.size(); i++)
{
	WS.verifyElementPropertyValue(response, 'data[i].employee_name', GlobalVariable.employeeList.data[i].name)
	WS.verifyElementPropertyValue(response, 'data[i].employee_salary', GlobalVariable.employeeList.data[i].salary)
	WS.verifyElementPropertyValue(response, 'data[i].employee_age', GlobalVariable.employeeList.data[i].age)
	
}



</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
