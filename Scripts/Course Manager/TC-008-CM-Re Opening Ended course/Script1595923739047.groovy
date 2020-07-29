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

WebUI.scrollToElement(findTestObject('Course Manager/button_DETAILS'), 0)

WebUI.click(findTestObject('Course Manager/button_DETAILS'))

WebUI.click(findTestObject('Course Manager/a_MODIFY'))

WebUI.click(findTestObject('Course Manager/a_Schedule'))

WebUI.click(findTestObject('Course Manager/input__generate_start_date'))

WebUI.click(findTestObject('Course Manager/div_29'))

WebUI.click(findTestObject('Course Manager/input__generate_end_date'))

WebUI.click(findTestObject('Course Manager/div_1'))

WebUI.click(findTestObject('Course Manager/button_Schedule'))

WebUI.verifyElementVisible(findTestObject('Course Manager/p_Updating course  Georgia TAPP Program 202_e049af'))

WebUI.click(findTestObject('Course Manager/a_Deadlines'))

if (findTestObject('Course Manager/label_Follow-up Completion date cannot be e_fd7ed2')) {
	WebUI.click(findTestObject('Course Manager/input_Evaluation Completion Date'))
} else {
	WebUI.click(findTestObject('Course Manager/label_Follow-up Completion date cannot be e_fd7ed2'))
}

WebUI.click(findTestObject('Course Manager/button_December_xdsoft_next'))

WebUI.click(findTestObject('Course Manager/div_2'))

WebUI.click(findTestObject('Course Manager/a_Follow-up Questions'))

WebUI.click(findTestObject('Course Manager/a_Finish'))

WebUI.verifyElementVisible(findTestObject('Course Manager/ul_Course updated successfully'))

WebUI.click(findTestObject('Course Manager/a_Course Manager'))

WebUI.click(findTestObject('Course Manager/a_Tools'))

WebUI.click(findTestObject('Course Manager/div_Course Enrollments'))

WebUI.click(findTestObject('Course Manager/a_Open Roster'))

WebUI.verifyElementVisible(findTestObject('Course Manager/li_Roster'))

WebUI.click(findTestObject('Course Manager/a_Tools'))

WebUI.click(findTestObject('Course Manager/div_Course Manager'))

WebUI.scrollToElement(findTestObject('Course Manager/p_Open'), 0)

WebUI.verifyElementVisible(findTestObject('Course Manager/p_Open'), FailureHandling.STOP_ON_FAILURE)

