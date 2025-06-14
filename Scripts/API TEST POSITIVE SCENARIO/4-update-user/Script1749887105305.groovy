import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

// get API response
def response = WS.sendRequest(findTestObject('API Method/4-update-user', [('baseUrl') : GlobalVariable.baseURL, ('apiKey') : GlobalVariable.apiKey, ('id_user') : GlobalVariable.idUser]))

// verify response code
WS.verifyResponseStatusCode(response, 200)

// get response data
def slurpler = new groovy.json.JsonSlurper()
def result = slurpler.parseText(response.getResponseBodyContent())


// verify user name
assert result.name != null
assert result.name instanceof String
assert result.name == 'Jeffry'


// verify user name
assert result.job != null
assert result.job instanceof String
assert result.job== 'QA Engineer'

