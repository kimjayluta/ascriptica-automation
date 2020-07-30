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

WebUI.click(findTestObject('Course Manager/select_Category'))

WebUI.verifyElementVisible(findTestObject('Course Manager/option_Learning Plan'))

WebUI.click(findTestObject('Course Manager/a_Course Manager'))

WebUI.click(findTestObject('Course Manager/a_Tools'))

WebUI.click(findTestObject('Course Manager/div_Manage Categories'))

WebUI.click(findTestObject('Course Manager/a_Unlimited_update_traning editstudent btn _a4b332'))

WebUI.click(findTestObject('Course Manager/div_ActiveInactive'))

WebUI.click(findTestObject('Course Manager/input_Delete_save_category'))

WebUI.click(findTestObject('Course Manager/a_Tools'))

WebUI.click(findTestObject('Course Manager/div_Course Manager'))

WebUI.click(findTestObject('Course Manager/button_New'))

WebUI.click(findTestObject('Course Manager/select_Category'))

