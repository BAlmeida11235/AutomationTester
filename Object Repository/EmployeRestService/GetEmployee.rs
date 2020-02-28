<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GetEmployee</name>
   <tag></tag>
   <elementGuidId>2d01e765-7ff5-4bae-83f0-4d72fde51590</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://dummy.restapiexample.com/api/v1/employee/${employee.data.id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.employee</defaultValue>
      <description></description>
      <id>c08e1db7-cab4-42da-a079-8dd63c6e0c88</id>
      <masked>false</masked>
      <name>employee</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

//Verify status code
WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

//Check response status
WS.verifyElementPropertyValue(response, 'status', &quot;success&quot;)

//Check if response is correct
WS.verifyElementPropertyValue(response, 'data.name', GlobalVariable.employee.data.name)
WS.verifyElementPropertyValue(response, 'data.salary', GlobalVariable.employee.data.salary)
WS.verifyElementPropertyValue(response, 'data.age', GlobalVariable.employee.data.age)
WS.verifyElementPropertyValue(response, 'data.id', GlobalVariable.employee.data.id)

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
