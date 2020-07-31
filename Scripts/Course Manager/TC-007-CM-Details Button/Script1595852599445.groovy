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

WebUI.click(findTestObject('Course Manager/button_DETAILS'))

WebUI.click(findTestObject('Course Manager/a_MODIFY'))

WebUI.click(findTestObject('Course Manager/a_Deadlines'))

WebUI.click(findTestObject('Course Manager/input_Evaluation Completion'))

WebUI.click(findTestObject('Course Manager/div_31 (August)'))

WebUI.click(findTestObject('Course Manager/a_Follow-up Questions'))

WebUI.click(findTestObject('Course Manager/a_Finish'))

WebUI.click(findTestObject('Course Manager/a_Course Manager'))

WebUI.scrollToElement(findTestObject('Course Manager/button_DETAILS'), 0)

WebUI.click(findTestObject('Course Manager/button_DETAILS'))

WebUI.click(findTestObject('Course Manager/a_DENY'))

WebUI.acceptAlert()

WebUI.setText(findTestObject('Course Manager/textarea_Reason For Decline_reason'), 'This is a Test')

WebUI.click(findTestObject('Course Manager/button_Submit (Deny)'))

WebUI.scrollToElement(findTestObject('Course Manager/p_Declined'), 0)

WebUI.verifyElementVisible(findTestObject('Course Manager/p_Declined'))

WebUI.click(findTestObject('Course Manager/button_DETAILS'))

WebUI.click(findTestObject('Course Manager/a_APPROVE'))

WebUI.acceptAlert()

WebUI.scrollToElement(findTestObject('Course Manager/a_Tools'), 0)

WebUI.click(findTestObject('Course Manager/a_Tools'))

WebUI.click(findTestObject('Course Manager/div_Course Manager'))

WebUI.click(findTestObject('Course Manager/button_DETAILS'))

WebUI.click(findTestObject('Course Manager/a_DUPLICATE'))

WebUI.selectOptionByValue(findTestObject('Course Manager/select_Choose                              _822a55'), '2', true)

WebUI.click(findTestObject('Course Manager/a_Follow-up Questions'))

WebUI.click(findTestObject('Course Manager/a_Finish'))

WebUI.click(findTestObject('Course Manager/a_Course Manager'))

WebUI.scrollToElement(findTestObject('Course Manager/button_DETAILS'), 0)

WebUI.click(findTestObject('Course Manager/button_DETAILS'))

WebUI.click(findTestObject('Course Manager/a_ENROLLMENT'))

WebUI.click(findTestObject('Course Manager/a_Course Manager'))

WebUI.click(findTestObject('Course Manager/button_DETAILS'))

WebUI.click(findTestObject('Course Manager/a_CANCEL'))

WebUI.acceptAlert()

WebUI.setText(findTestObject('Course Manager/textarea_Reason For Cancel_reason_cancel'), 'This is a test')

WebUI.click(findTestObject('Course Manager/button_Submit_Cancel'))

WebUI.scrollToElement(findTestObject('Object Repository/Course Manager/p_Cancelled'), 0)

WebUI.verifyElementVisible(findTestObject('Object Repository/Course Manager/p_Cancelled'))

WebUI.click(findTestObject('Course Manager/button_DETAILS'))

WebUI.click(findTestObject('Course Manager/a_DELETE'))

WebUI.acceptAlert()

