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

WebUI.setText(findTestObject('Object Repository/Course Manager/input_User ID  Email Address_LoginFormusername'), 'jeffdy010203@gmail.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Course Manager/input_Password_LoginFormpassword'), 'mpwtaF1AOAT+pVLiPGdxib800znL7mVX')

WebUI.click(findTestObject('Object Repository/Course Manager/button_Login'))

WebUI.click(findTestObject('Course Manager/a_Tools'))

WebUI.click(findTestObject('Course Manager/div_Course Manager'))

WebUI.scrollToElement(findTestObject('Course Manager/button_New'), 0)

WebUI.click(findTestObject('Course Manager/button_New'))

WebUI.scrollToElement(findTestObject('Course Manager/a_Next'), 0)

WebUI.click(findTestObject('Object Repository/Course Manager/a_Next'))

WebUI.scrollToElement(findTestObject('Course Manager/label_This field is required'), 0)

WebUI.verifyElementVisible(findTestObject('Course Manager/label_This field is required'))

WebUI.verifyElementVisible(findTestObject('Course Manager/label_This field is required_1'))

WebUI.scrollToElement(findTestObject('Object Repository/Course Manager/label_This field is required_1_2'), 0)

WebUI.verifyElementVisible(findTestObject('Object Repository/Course Manager/label_This field is required_1_2'))

WebUI.scrollToElement(findTestObject('Object Repository/Course Manager/input__txtTitle'), 0)

WebUI.setText(findTestObject('Object Repository/Course Manager/input__txtTitle'), 'Automation 101')

WebUI.selectOptionByValue(findTestObject('Object Repository/Course Manager/select_Choose                              _822a55'), 
    '2', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Course Manager/select_ChooseBlended Learning (OnlineFace-t_17004a'), 
    '176', true)

WebUI.scrollToElement(findTestObject('Course Manager/select_Delivery Methods'), 0, FailureHandling.STOP_ON_FAILURE)

WebUI.selectOptionByValue(findTestObject('Course Manager/select_Delivery Methods'), '12', true)

WebUI.click(findTestObject('Object Repository/Course Manager/a_Next'))

WebUI.verifyElementVisible(findTestObject('Object Repository/Course Manager/span_Description'))

