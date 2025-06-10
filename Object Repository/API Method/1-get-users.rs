<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>1-get-users</name>
   <tag></tag>
   <elementGuidId>ce8b411e-56aa-4aa1-b7ff-dafadff8f69a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-api-key</name>
      <type>Main</type>
      <value>${apiKey}</value>
      <webElementGuid>b45cf7e9-0972-4db7-8298-fff3a769b6e8</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.2.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${baseUrl}/users?page=${page}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>2</defaultValue>
      <description></description>
      <id>499d1408-fb4b-471b-8468-09f3f2a9eefa</id>
      <masked>false</masked>
      <name>page</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.baseURL</defaultValue>
      <description></description>
      <id>b78e00c8-5f65-476a-a4a1-b806eb96826a</id>
      <masked>false</masked>
      <name>baseUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.apiKey</defaultValue>
      <description></description>
      <id>32cc4063-b01b-4ced-804f-f03ca045e485</id>
      <masked>false</masked>
      <name>apiKey</name>
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

def slurpler = new groovy.json.JsonSlurper()
def result = slurpler.parseText(response.getResponseBodyContent())
def firstName = result.data[0].first_name
def lastName = result.data[0].last_name

def variables = request.getVariables()
def pageApi = variables.get('page')


// verify response code
WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

// verify page
WS.verifyElementPropertyValue(response, 'page', pageApi)

// verify a users data
assert firstName != null
assert firstName instanceof String
assert firstName == GlobalVariable.firstName
assert lastName == GlobalVariable.lastName

WS.verifyElementPropertyValue(response, 'data[0].id', 7)
WS.verifyElementPropertyValue(response, 'data[0].avatar', 'https://reqres.in/img/faces/7-image.jpg')

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
