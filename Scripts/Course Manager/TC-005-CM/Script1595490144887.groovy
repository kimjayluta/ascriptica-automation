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

WebUI.setText(findTestObject('Course Manager/input_User ID (EXACT)'), 'jeffdy010203@gmail.com')

WebUI.setEncryptedText(findTestObject('Course Manager/input_Password (EXACT)'), 'mpwtaF1AOAT+pVLiPGdxib800znL7mVX')

WebUI.click(findTestObject('Course Manager/button_Login'))

WebUI.click(findTestObject('Course Manager/a_Tools'))

WebUI.click(findTestObject('Course Manager/div_Course Manager'))

WebUI.click(findTestObject('Course Manager/button_New'))

WebUI.setText(findTestObject('Course Manager/input__txtTitle'), 'Comp-101')

WebUI.selectOptionByValue(findTestObject('Object Repository/Course Manager/select_Choose                              _822a55'), 
    '2', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Course Manager/select_ChooseBlended Learning (OnlineFace-t_17004a'), 
    '175', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Course Manager/select_Choose                              _ed4c48'), 
    '11', true)

WebUI.click(findTestObject('Course Manager/a_Next'))

WebUI.delay(2)

CustomKeywords.'test.fileUpload.uploadFile'(findTestObject('Course Manager/input_BROWSE_test'), 'C:\\Users\\Kim\\Downloads\\depositphotos_180736330-stock-illustration-logo-quality-assurance-qa-testing.jpg')

WebUI.setText(findTestObject('Course Manager/textarea_Course Desciption'), 'This is a test description')

WebUI.click(findTestObject('Course Manager/a_Next'))

