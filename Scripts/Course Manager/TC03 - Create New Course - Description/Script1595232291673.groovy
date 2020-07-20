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

WebUI.openBrowser('')

WebUI.maximizeWindow()

WebUI.navigateToUrl('http://demos1.stral.in/PGCTechnologyServices/site/login')

WebUI.setText(findTestObject('Course Manager/input_Username'), 'jeffdy010203@gmail.com')

WebUI.setEncryptedText(findTestObject('Course Manager/input_Password'), 'mpwtaF1AOAT+pVLiPGdxib800znL7mVX')

WebUI.click(findTestObject('Course Manager/button_Login'))

WebUI.click(findTestObject('Course Manager/a_Tools_EXACT'))

WebUI.click(findTestObject('Course Manager/div_Course Manager'))

WebUI.click(findTestObject('Course Manager/button_New'))

WebUI.setText(findTestObject('Course Manager/input__txtTitle'), 'COMP-101')

WebUI.selectOptionByValue(findTestObject('Course Manager/select_Choose_822a55'), '12', true)

WebUI.click(findTestObject('Course Manager/li_Delivery Methods_9b9150'))

WebUI.selectOptionByValue(findTestObject('Course Manager/select_Choose_ed4c48'), '9', true)

WebUI.click(findTestObject('Course Manager/a_Next'))

CustomKeywords.'test.fileUpload.uploadFile'(findTestObject('Course Manager/input_Upload button (Description)'), 'C:\\Users\\Kim\\Downloads\\Test-seller-a3-94750.png')

WebUI.delay(3)

WebUI.setText(findTestObject('Course Manager/p'), 'This is a test description from Automation.')

WebUI.click(findTestObject('Course Manager/a_Next'))

