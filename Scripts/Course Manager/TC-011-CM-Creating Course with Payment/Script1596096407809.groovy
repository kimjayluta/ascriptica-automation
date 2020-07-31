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

WebUI.delay(2)

WebUI.click(findTestObject('Course Manager/a_ Add Instructor'))

WebUI.setText(findTestObject('Course Manager/input__txtInstructor'), 'Lauro E')

WebUI.click(findTestObject('Course Manager/a_Lauro Eguia'))

WebUI.delay(3)

WebUI.click(findTestObject('Course Manager/button_Submit'))

WebUI.click(findTestObject('Course Manager/a_Next'))

WebUI.click(findTestObject('Course Manager/select_Training Center'))

WebUI.click(findTestObject('Course Manager/Train_center_Value'))

WebUI.click(findTestObject('Course Manager/select_Room'))

WebUI.click(findTestObject('Course Manager/Room_Value'))

WebUI.click(findTestObject('Course Manager/a_Next'))

WebUI.click(findTestObject('Course Manager/a_ Add Focus Area'))

WebUI.click(findTestObject('Course Manager/option_Certification'))

WebUI.click(findTestObject('Course Manager/button_Submit_FocusArea'))

WebUI.click(findTestObject('Course Manager/a_Next'))

WebUI.click(findTestObject('Course Manager/select_TextBookProvided'))

WebUI.click(findTestObject('Course Manager/option_Yes'))

WebUI.setText(findTestObject('Course Manager/input_SD_custFiled_30'), '1234512345')

WebUI.click(findTestObject('Course Manager/input_Migrated Course_custFiled_31'))

WebUI.setText(findTestObject('Course Manager/textarea_Learning Outcomes_custFiled_29'), 'This is a test from automation')

WebUI.click(findTestObject('Course Manager/a_Next'))

WebUI.click(findTestObject('Course Manager/a_Next'))

WebUI.scrollToElement(findTestObject('Course Manager/input_Seats Expire On_DTRsvSeat'), 0)

WebUI.click(findTestObject('Course Manager/input_Seats Expire On_DTRsvSeat'))

WebUI.click(findTestObject('Course Manager/button_December_xdsoft_next (Limits)'))

WebUI.click(findTestObject('Course Manager/div_31 (Expiry Date)'))

WebUI.click(findTestObject('Course Manager/a_Next'))

WebUI.setText(findTestObject('Course Manager/input_Non-Member Fee'), '50.00')

WebUI.setText(findTestObject('Course Manager/input_Member Fee'), '100.00')

WebUI.setText(findTestObject('Course Manager/input_Materials Fee'), '20.00')

WebUI.click(findTestObject('Course Manager/a_Next'))

WebUI.click(findTestObject('Course Manager/input__generate_start_date'))

WebUI.click(findTestObject('Course Manager/button_December_xdsoft_next'))

WebUI.click(findTestObject('Course Manager/div_1 (Sched Start Date)'))

WebUI.click(findTestObject('Course Manager/input__generate_end_date'))

WebUI.click(findTestObject('Course Manager/button_December_xdsoft_next (End Date)'))

WebUI.click(findTestObject('Course Manager/div_31 (Sched End Date)'))

WebUI.click(findTestObject('Course Manager/button_Schedule'))

WebUI.click(findTestObject('Course Manager/a_Next'))

WebUI.click(findTestObject('Course Manager/a_Next'))

WebUI.click(findTestObject('Course Manager/a_ Add Restriction'))

WebUI.setText(findTestObject('Course Manager/input__txtClassEnrollGroupName'), 'TEST')

WebUI.click(findTestObject('Course Manager/option_a_GG test'), FailureHandling.STOP_ON_FAILURE)

WebUI.setText(findTestObject('Course Manager/input__ClassEnrollGroupLimit'), '10')

WebUI.click(findTestObject('Course Manager/input_Remove Limit After_dtExpiryDate'), FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Course Manager/div_30'), FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Course Manager/button_Submit (Restriction)'), FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Course Manager/a_Next'))

WebUI.click(findTestObject('Course Manager/a_ Target Audience'))

WebUI.click(findTestObject('Course Manager/option_Paraprofessionals'))

WebUI.click(findTestObject('Course Manager/button_Submit (Target Audience)'))

WebUI.click(findTestObject('Course Manager/a_Next'))

WebUI.click(findTestObject('Course Manager/a_ Rubric'))

WebUI.click(findTestObject('Course Manager/select_Rubric'))

WebUI.click(findTestObject('Course Manager/option_Admin'))

WebUI.click(findTestObject('Course Manager/select_Domain'))

WebUI.click(findTestObject('Course Manager/option_Planning'))

WebUI.click(findTestObject('Course Manager/option_sample'), FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Course Manager/button_Submit (Rubric)'))

WebUI.click(findTestObject('Course Manager/a_Next'))

WebUI.click(findTestObject('Course Manager/a_Next'))

WebUI.click(findTestObject('Course Manager/a_Next'))

WebUI.click(findTestObject('Course Manager/a_Finish'))

WebUI.click(findTestObject('Course Manager/a_Course Manager'))

WebUI.delay(2)

WebUI.closeBrowser()

