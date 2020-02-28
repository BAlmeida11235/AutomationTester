<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateEmployee</name>
   <tag></tag>
   <elementGuidId>9b9b7e80-bb8d-4ad1-a682-b5dbc2825f93</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;name\&quot;:\&quot;${employeeName}\&quot;,\n  \t\&quot;salary\&quot;:\&quot;${employeeSalary}\&quot;,\n  \t\&quot;age\&quot;:\&quot;${employeeAge}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://dummy.restapiexample.com/api/v1/create</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.employee</defaultValue>
      <description>create employee response</description>
      <id>f101a261-87ff-4f32-8a95-31bca5e832bd</id>
      <masked>false</masked>
      <name>employee</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.employeeList</defaultValue>
      <description></description>
      <id>ecdfc41c-53b0-428d-ae2d-a4f0b1d25680</id>
      <masked>false</masked>
      <name>employeeList</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>0ca8a6ba-5024-4b1a-88b1-eb59f2431e90</id>
      <masked>false</masked>
      <name>employeeName</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>060b7f18-eede-43c6-9798-45a8325e3f77</id>
      <masked>false</masked>
      <name>employeeSalary</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3be00b05-df03-455b-afd1-bc8e2059f202</id>
      <masked>false</masked>
      <name>employeeAge</name>
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

//Parse response Json format
def slurper = new groovy.json.JsonSlurper()
responseBody = slurper.parseText(response.getResponseBodyContent())
GlobalVariable.employee= responseBody

//Test data from the response matches data from the request
WS.verifyElementPropertyValue(response, 'data.name', GlobalVariable.employee.data.name)
WS.verifyElementPropertyValue(response, 'data.salary', GlobalVariable.employee.data.salary)
WS.verifyElementPropertyValue(response, 'data.age', GlobalVariable.employee.data.age)

//Test for invalid input. Empty, null, special characters
//WS.verifyElementPropertyValue(response, 'data.name', !null)
//WS.verifyElementPropertyValue(response, 'data.name', !'')

/*
String[] arrayResponse = [&quot;why&quot;, &quot;hello&quot;, &quot;there&quot;]
String[] arrayExpect = [&quot;there&quot;, &quot;why&quot;, &quot;hello&quot;]
assertThat(arrayResponse).containsOnly(&quot;there&quot;, &quot;hello&quot;, &quot;why&quot;)
assertThat(arrayResponse).containsOnlyElementsOf(Arrays.asList(&quot;why&quot;, &quot;there&quot;, &quot;hello&quot;))

assertThat(arrayResponse).containsExactly(&quot;why&quot;, &quot;hello&quot;, &quot;there&quot;)
assertThat(arrayResponse).containsExactlyElementsOf(Arrays.asList(&quot;why&quot;, &quot;hello&quot;, &quot;there&quot;))

assertThat(arrayResponse).containsSequence(&quot;why&quot;, &quot;hello&quot;)
assertThat(arrayResponse).containsSubsequence(&quot;why&quot;, &quot;there&quot;)
assertThat(arrayResponse).containsAnyOf(&quot;why&quot;, &quot;nothing&quot;, &quot;new&quot;)

assertThat(arrayResponse).contains(&quot;hello&quot;, atIndex(1))


assertThat(response.getResponseText()).isEqualTo(&quot;Katalon Test Project&quot;)
*/

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
