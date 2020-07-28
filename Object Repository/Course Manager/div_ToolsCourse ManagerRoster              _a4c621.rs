<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_ToolsCourse ManagerRoster              _a4c621</name>
   <tag></tag>
   <elementGuidId>eac28277-2178-47e0-9b75-c1d49f5dca28</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>(.//*[normalize-space(text()) and normalize-space(.)='Logout'])[1]/following::div[10]</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>course-inner-cont course-manager</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
              Tools
Course Manager
Roster
        
            
                
                
                
                
                    Georgia TAPP Program 2020 VIII 
                     
                    
                        
                            Tue, Jul 28, 2020 - Fri, Jul 31, 2020
                        
                        
                            GWCC
                        
                        
                            Room C
                        
                    
                
            
            
                                    
                    
                        35
                        AVAILABLE SEATS
                        Enrolled : 0
                    
                
                
                    
                        
                            
                                 Name tag data export
                            
                             
                                                                    

                                        No Participants found

                                    

                                                                

                                    $('#frm_tags_name').validate({

                                        onfocusout: false, //Disables onblur validation.
                                        onkeyup :false,  //Disables onkeyup validation
                                        onclick : false, //Disables onclick validation of checkboxes and radio buttons.
                                        rules: {

                                                for_validation:{
                                                        required: function(elem)
                                                        {
                                                           return $('input[name=&quot;export_fields[]&quot;]:checked').length &lt;= 0;
                                                        }
                                                    }



                                        },
                                         errorPlacement: function(error, element) {


                                                   error.appendTo( element.next());

                                          }

                                    });


                                
                             

                        
                    
                    
                 
                                    
                        
                          
                            
                              NON-MEMBERS
                              $500
                            
                            
                              MEMBERS
                              $300
                            
                          
                                                                                MODIFY

                                                                      
                      
                                
            
        
        
            
                
                    
                    Instructor Detail
                                                             
                    
                        
                             
                                                                            
                                                
                                           
                                                                                                                        
                                           
                                                                           
                            
                                                                       
                                           
                                                                                  
                                               
                                            
                                            Peter Windsor
                                            Instructor
                                         
                                    
                                 
                                 
                        
                       
                        
                                     

                    
                        
                          
                        
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   706-865-2141-265                        
                        
                        ETCregistration@pgccountyschools.com
                    
                
                
                    Details
                    Course No.
                    #3267
                    Category
                    District : ETC
                    Classroom Hours
                    10
                    Non-ClassroomHours
                    1
                    Component
                    -
                    
                    Follow-up Method
                    Blended Classroom  (Online/Classroom)
                    
                    Evaluation Method
                    None Required
                    
                    Delivery Method
                    Blended Classroom  (Online/Classroom)
                
                
                    Toggle
                    ▲
                    ▼
                
            

            
                
                    
                        
                                                            
                                    
                                        Already Started
                                    
                                    -

                                
                            
                            
                                
                                
                                    
                                        Start Date
                                    
                                    
                                        Tue, Jul 28, 2020
                                    
                                
                                
                                    
                                        End Date
                                    
                                    
                                        Fri, Jul 31, 2020
                                    
                                
                                
                            
                        
                    
                    
                        
                            
                            
                                
                                    Registration Deadline
                                
                                
                                    Tue, Jul 28, 2020
                                
                            
                            
                                
                                    Withdrawal Deadline
                                
                                
                                    Tue, Jul 21, 2020
                                
                            
                            
                        
                    
                
                
                      
                            Participants                                
                                                                                     
                                    
                                    
                                   
                           
                                                            
                                     
                                   
                                                                                    
                                                                                                         
                                                                                  
                                         
                                           
                                     
                                                                                                                      
                                         
                                        
                                        
                                    
                                   
                                   
                             
                                          
                            
                                
                                  Search 
                                
                                    
                                     function doSearch(){
                                         
                                           $(&quot;#search_link&quot;).attr('href',$(&quot;#search_link&quot;).attr('href')+'&amp;search='+$.trim($(&quot;#txt_searh&quot;).val()));
                                           $(&quot;#search_link&quot;).trigger('click');
                                           
                                         
                                     }
                                      $(&quot;#btn_searh&quot;).click(function(){

                                           doSearch()
                                      });
                                      $('#txt_searh').on(&quot;keypress&quot;, function(e) {
                                          
                                           if (e.keyCode == 13) {
                                               
                                               doSearch()
                                           }
                                         
                                       });
                                
                                    
                                Actions
                                
                                    
                                    Toggle Dropdown
                                
                              
                                
                                                                           
                                                                                     Send Message
                                            
                                                                                     Create Sign-In List
                                         
                                            
                                             Generate Certificates
                                         
                                            
                                             Name tag data export
                                           
                                            
                                                
                                                                                            
                                                        
                                                                                                                    Evaluation Summary
                                                            
                                                       
                                                                    $('.popup-with-form').magnificPopup({
                                                                       type: 'inline',
                                                                       preloader: false,
                                                                       focus: '#name',
                                                                       closeOnBgClick:false,
                                                                       enableEscapeKey:false
                                                                   });


                                                                   
                                                     

                                                    
                                            
                                            
                                                                          
                                    
                                             
                               
                                                                             Enrollment Export
                                        
                                    
                                                                                                                            Mass Update
                                                                               
                                                                                   
                                                                                         Re-send work location approval 
                                             
                                            
                                               Re-send district approval 
                                            
                                            

                                           

                                             
                                              Re-send confirmation      
                                             
                                         
     
                                       
                                
                                
                                    
                            
                     
                          
                        
                    
                         
                                        
                      
                          
                                    
                                      
                                    
                                    function evalsummury(){ 
                                        
                                        var isWR=0;
                                        var ms=&quot;Omit written responses?&quot;; 
                                        r=confirm(ms);

                                        if(r==true){
                                            
                                            isWR=1;
                                        } 
                                         
                                       var redirectTo='/PGCTechnologyServices/course-enrollments/evaluationsummary?classid=3267&amp;isWR='+isWR;
                                      // window.location.href=redirectTo;
                                      
                                      download_url_eval_evaluationsummary(redirectTo);
                                     }
                                     
                                     function download_url_eval_evaluationsummary(urlToSend){


                                        waitingDialog.show('Generating Evaluation Summary....');
                                        var req = new XMLHttpRequest();
                                             req.open(&quot;GET&quot;, urlToSend, true);
                                             //req.responseType = &quot;text/html&quot;;
                                             req.onload = function (event) {
                                                 var blob = req.response;

                                                 var fileName = 'evaluationsummary_3267.xls' //if you have the fileName header available

                                                 waitingDialog.hide();

                                               if (/^[\],:{}\s]*$/.test(blob.replace(/\\[&quot;\\\/bfnrtu]/g, '@').replace(/&quot;[^&quot;\\\n\r]*&quot;|true|false|null|-?\d+(?:\.\d*)?(?:[eE][+\-]?\d+)?/g, ']').replace(/(?:^|:|,)(?:\s*\[)+/g, ''))) {
                                                    var response=jQuery.parseJSON(blob);
                                               }
                                                if(typeof response =='object'){

                                                    alert(response.message);
                                                }
                                                else{

                                                      const blb    = new Blob([blob], {type: &quot;text/html&quot;});
                                                      const reader = new FileReader();

                                                       // This fires after the blob has been read/loaded.
                                                       reader.addEventListener('loadend', (e) => {
                                                          download(blb,fileName);
                                                       });

                                                       // Start reading the blob as text.
                                                       reader.readAsText(blb);
                                                }
                                             };

                                             req.send();
                                            
                                    }
                                     
                                     function certificate_management(){
                                         
                                         
                                           $.magnificPopup.open({
                                                    items: {
                                                        src: '#certificate-management' 
                                                    },
                                                    preloader: false,
                                                    type: 'inline',
                                                    closeOnBgClick:false,
                                                    enableEscapeKey:false,
                                                    callbacks:  {
                                                                    open: function() {

                                                                        clickGenerateAll=false;
                                                                        $(&quot;#to_email&quot;).val('');
                                                                        $(&quot;#to_email_id&quot;).val('');
                                                                        $(&quot;#cc&quot;).val('');
                                                                        $(&quot;#cc_id&quot;).val('');
                                                                        $(&quot;#subject&quot;).val('');
                                                                        $(&quot;#checked_participants&quot;).val('');
                                                                        $(&quot;#is_checked_all&quot;).val('');
                                                                        $(&quot;#unchecked_participants&quot;).val('');
                                                                        
                                                                         $(&quot;#chk_all&quot;).prop(&quot;checked&quot; , false);
                                                                         $('.chkboxs input[type=&quot;checkbox&quot;]').each(function(){
                                                                                $(this).prop(&quot;checked&quot; , false);
                                                                          }) 
                                                                            
                                                                      },
                                                                     close: function(){

                                                                     
                                                                     


                                                                     } 

                                                                   }
                                                });
                                         
                                     }
                                     
                                     function openSendEmailForm(){
                                         
                                           $(&quot;.morefiles&quot;).empty(); 
                                           $(&quot;#mainAttachment&quot;).val(''); 
                                           $(&quot;#ccemails&quot;).val(''); 
                                           $.magnificPopup.open({
                                                    items: {
                                                        src: '#send-message' 
                                                    },
                                                    preloader: false,
                                                    type: 'inline',
                                                    closeOnBgClick:false,
                                                    enableEscapeKey:false,
                                                    callbacks:  {
                                                                    open: function() {


                                                                      tinymce.init({
                                                                                            selector: '#txtmsg',
                                                                                            setup: function (editor) {
                                                                                                            editor.on('change', function () {
                                                                                                                editor.save();
                                                                                                            }),
                                                                                                            editor.on('init', function(args) {
                                                                                                                
                                                                                                                
                                                                                                                tinyMCE.activeEditor.setContent('');
                                                                                                               // tinyMCE.activeEditor.focus();
                                                                                                                $(&quot;#rgSchedule_immidiatly&quot;).trigger('click');
                                                                                                                $(&quot;#txtsubject&quot;).val('');
                                                                                                                
                                                                                                            })       
                                                                                                            
                                                                                                            

                                                                                                        },
                                                                                            height: 150,
                                                                                            width:400,
                                                                                            menubar: false,
                                                                                            plugins: [
                                                                                              'advlist autolink lists charmap print preview anchor',
                                                                                              'searchreplace visualblocks code fullscreen',
                                                                                              'insertdatetime  table contextmenu paste code',
                                                                                              'textcolor'
                                                                                            ],
                                                                                            toolbar: 'forecolor backcolor | fontselect fontsizeselect | undo redo | insert | styleselect | bold italic  | alignleft aligncenter alignright alignjustify | bullist numlist outdent indent',
                                                                                            fontsize_formats: &quot;8pt 10pt 12pt 14pt 18pt 24pt 36pt&quot;,
                                                                                            textcolor_cols: &quot;5&quot;

                                                                                          });


                                                                                            $(&quot;#send_on&quot;).addClass('disabled_back');
                                                                                            $(&quot;#send_on&quot;).attr('disabled','disabled');

                                                                                            
                                                                                            $(&quot;#to_all&quot;).hide();
                                                                                            $(&quot;#multiparticipants&quot;).hide();
                                                                                            $(&quot;.ms-options-wrap&quot;).show();
                                                                                             $('#multiparticipants').multiselect({
                                                                                                    columns: 2,
                                                                                                    selectAll:true,
                                                                                                    placeholder: 'Participants',
                                                                                                    onLoad : function( element, option ){
                                                                                                         $(&quot;.ms-selectall&quot;).trigger('click');
                                                                                                         //$(&quot;.ms-selectall&quot;).text('Unselect All');
                                                                                                    },
                                                                                                    onOptionClick : function(){
                                                                                                        if($(&quot;#multiparticipants option:selected&quot;).length>=0){

                                                                                                            $(&quot;.ms-selectall&quot;).text('Select All');
                                                                                                        }
                                                                                                        else if($(&quot;#multiparticipants option:selected&quot;).length==$(&quot;#multiparticipants option&quot;).length){

                                                                                                            $(&quot;.ms-selectall&quot;).text('Unselect All');
                                                                                                        }

                                                                                                    }
                                                                                                });
                                                                                            

                                                                      },
                                                                     close: function(){

                                                                     
                                                                        tinymce.remove(&quot;#txtmsg&quot;);



                                                                     } 

                                                                   }
                                                });
                                                                                                
                                       
                                         
                                     }
                                      var download = function(blob, filename) {
                                          
                                        if (navigator.appVersion.toString().indexOf('.NET') > 0){
                                               window.navigator.msSaveBlob(blob, filename);
                                        }
                                        /*else if(navigator.userAgent.indexOf(&quot;Safari&quot;) > -1){
                                            
                                            alert(&quot;hi&quot;);
                                            window.open('data:application/pdf;charset=utf-8,' + encodeURIComponent(blob));

                                        }*/
                                        else{
                                                    var link = document.createElement('a');
                                                    link.href = URL.createObjectURL(blob);
                                                    link.download = filename;
                                                    document.body.appendChild(link);
                                                    link.click();
                                                    // Revoking is tricky:
                                                    // https://code.google.com/p/chromium/issues/detail?id=375297#c7
                                                    window.setTimeout(function() {
                                                      URL.revokeObjectURL(blob);
                                                      document.body.removeChild(link);
                                                    }, 0);
                                        
                                            }
                                      };
                                      
                                    function download_url(urlToSend){
                                       
                                       var isSafari = /constructor/i.test(window.HTMLElement) || (function (p) { return p.toString() === &quot;[object SafariRemoteNotification]&quot;; })(!window['safari'] || (typeof safari !== 'undefined' &amp;&amp; safari.pushNotification));
                                       if(isSafari){
                                           
                                           window.location.href=urlToSend;
                                       }
                                       else{
                                            waitingDialog.show('Generating SignIn Sheet....');
                                            var req = new XMLHttpRequest();
                                                 req.open(&quot;GET&quot;, urlToSend, true);
                                                 req.responseType = &quot;blob&quot;;
                                                 req.onload = function (event) {
                                                     
                                                      if (req.status != 200) {
                                                          
                                                            alert('Access denied. You have no permission to generate signin sheet'); // e.g. 404: Not Found
                                                            waitingDialog.hide();
                                                            
                                                      } else { // show the result

                                                            var blob = req.response;
                                                            var fileName = 'SignIn-georgia-tapp-program-2020-viii.pdf' //if you have the fileName header available

                                                            waitingDialog.hide();
                                                            /*
                                                            var link=$(&quot;#hiddenDownload&quot;);
                                                            link.href=window.URL.createObjectURL(blob);
                                                            link.download=fileName;
                                                            link.click();
                                                             waitingDialog.hide();
                                                             */

                                                            download(blob,fileName);
                                                        }
                                                 };
                                                 
                                                 req.send();
                                            }  
                                    }

                                  function ExtendEvalDate(regid,studentName){
                                  
                                        var ms=&quot;Would you like to extend evaluation closing for this &quot;+studentName+&quot; for 24hours?&quot;; 
                                        r=confirm(ms);
                                        if(r===true){
                                         $.ajax({
                                                    type: &quot;POST&quot;,
                                                    cache: false,
                                                    timeout:600000,
                                                    url: &quot;/PGCTechnologyServices/course-enrollments/extendevaldate&quot;,
                                                    data: {class_id:3267,RegId:regid},
                                                    beforeSend:function() {

                                                           //waitingDialog.hide();
                                                           //$('.ajax_instr_loader').hide();
                                                           //waitingDialog.show('Loading....');
                                                     },
                                                     complete: function() {

                                                           //waitingDialog.hide();
                                                           //$('.ajax_instr_loader').hide();
                                                       },        
                                                     success: function(data) {

                                                        data=jQuery.parseJSON(data);
                                                        waitingDialog.hide(); 
                                                        alert(data.message);  

                                                           

                                                      },
                                                        error: function (xhr, ajaxOptions, thrownError) {
                                                               //alert(xhr.status);
                                                               //alert(thrownError);
                                                               //waitingDialog.hide();
                                                               waitingDialog.hide();
                                                             
                                                         }

                                                   })
                                  
                                            }      
                                  }
                                    
                                
                     
                                                    
                         Search
                         
                         
                         
                            
                                   

                                                    
                         count=0;groupmem=0;
                        
                            
                                
                                    
                                        
                                                                                                     
                
                                                    
                                                                                                                 
                                                            
                                                        Name 
                                                    
                                                
                                             
                                                                                                PLUs 
                                          
                                          
                                            
                                                                                                Completed 
                                          
                                                                                    
                                                                                                Paid 
                                          
                                                                                                                                
                                                                                                    Evaluation Status 
                                            
                                            
                                        
                                                                                    Registration Status 
                                          
                                         
                                    
                                
                                
                                  
                                    
                                          
                                
                                        
                                             No Participants Found
                                            
                                        
                                          
                                
                                
                            
                              
                                $(&quot;div#new-model&quot;).on('click', '#selectall_', function() {
                                   
                                   if($(this).is(':checked')){
                                       
                                        $('.resendConfirmation').each(function(){
                                             $(this).prop(&quot;checked&quot; , true);
                                        }) 
                                   }
                                   else{
                                       
                                       $('.resendConfirmation').each(function(){
                                             $(this).prop(&quot;checked&quot; , false);
                                        }) 

                                   }
                                });
                                
                                 $('#resendworklocation_approval').on(&quot;click&quot;, function(e) {
                                          
                                        var checked_itm=$('.resendConfirmation:checked').map(function() {return this.value;}).get().join(',');
                                         if(checked_itm==&quot;&quot;){

                                             alert(&quot;Please select atleast one participant&quot;);
                                         }
                                        else{
                                            
                                                flag=true;  
                                                var splitedClassArr=  checked_itm.split(&quot;,&quot;);
                                                $.each(splitedClassArr, function( index, value ) {
                                                    
                                                    
                                                    var elm=$(&quot;#parti_&quot;+value);
                                                    if(elm.length>0){
                                                    
                                                        locreqapprove=$(elm).attr('data-locreqapprove');
                                                        statusid=$(elm).attr('data-statusid');
                                                        if(locreqapprove==&quot;1&quot; || statusid==&quot;3&quot;){
                                                        
                                                            flag=false;
                                                            alert(&quot;Your selection contain already approved participant or already confirmed participant &quot;)
                                                            return false;
                                                        }
                                                    
                                                    }
                         
                                                    
                                               });    
                                               
                                               if(flag==true){
                                               
                                                       var datastring={ participant_ids:checked_itm,classid:3267} ; 
                                                        $.ajax({
                                                              type: &quot;POST&quot;,
                                                              cache: false,
                                                              url: &quot;/PGCTechnologyServices/course-enrollments/resendworklocationapproval&quot;,
                                                              data: datastring,
                                                              success: function(data) {

                                                                    data=jQuery.parseJSON(data);
                                                                    alert(data.msg);
                                                                 
                                                              },
                                                               error: function (xhr, ajaxOptions, thrownError) {
                                                                  alert(xhr.status);
                                                                  alert(thrownError);
                                                                  //waitingDialog.hide();
                                                                   waitingDialog.hide(); 
                                                                }

                                                          });
                                                
                                               }

                                        }
                                 });
                                 
                                 $('#resend_confirmation').on(&quot;click&quot;, function(e) {
                                          
                                        var checked_itm=$('.resendConfirmation:checked').map(function() {return this.value;}).get().join(',');
                                         if(checked_itm==&quot;&quot;){

                                             alert(&quot;Please select atleast one participant&quot;);
                                         }
                                        else{
                                            
                                                   var datastring={ participant_ids:checked_itm,classid:3267} ; 
                                                    $.ajax({
                                                          type: &quot;POST&quot;,
                                                          cache: false,
                                                          url: &quot;/PGCTechnologyServices/course-enrollments/resendconfirmation&quot;,
                                                          data: datastring,
                                                          success: function(data) {

                                                                data=jQuery.parseJSON(data);
                                                                alert(data.msg);

                                                          },
                                                           error: function (xhr, ajaxOptions, thrownError) {
                                                              alert(xhr.status);
                                                              alert(thrownError);
                                                              //waitingDialog.hide();
                                                               waitingDialog.hide(); 
                                                            }

                                                      });

                                        }
                                 });
                                 
                                 $('#resenddistrict_approval').on(&quot;click&quot;, function(e) {
                                          
                                        var checked_itm=$('.resendConfirmation:checked').map(function() {return this.value;}).get().join(',');
                                         if(checked_itm==&quot;&quot;){

                                             alert(&quot;Please select atleast one participant&quot;);
                                         }
                                        else{
                                            
                                                flag=true;  
                                                var splitedClassArr=  checked_itm.split(&quot;,&quot;);
                                                $.each(splitedClassArr, function( index, value ) {
                                                    
                                                    
                                                    var elm=$(&quot;#parti_&quot;+value);
                                                    if(elm.length>0){
                                                    
                                                        locreqapprove=$(elm).attr('data-locreqapprove');
                                                        distreqapprove=$(elm).attr('data-distreqapprove');
                                                        statusid=$(elm).attr('data-statusid');
                                                        if(locreqapprove==&quot;0&quot;){
                                                            
                                                            flag=false;
                                                            alert(&quot;Your selection contain participants that not yet marked approved for work location&quot;)
                                                            return false; 
                                                        }
                                                        if(distreqapprove==&quot;1&quot; || statusid==&quot;3&quot;){
                                                        
                                                            flag=false;
                                                            alert(&quot;Your selection contain already approved participant or already confirmed participant &quot;)
                                                            return false;
                                                        }
                                                    
                                                    }
                         
                                                    
                                               });    
                                               
                                               if(flag==true){
                                               
                                                       var datastring={ participant_ids:checked_itm,classid:3267} ; 
                                                        $.ajax({
                                                              type: &quot;POST&quot;,
                                                              cache: false,
                                                              url: &quot;/PGCTechnologyServices/course-enrollments/resenddistrictapproval&quot;,
                                                              data: datastring,
                                                              success: function(data) {

                                                                    data=jQuery.parseJSON(data);
                                                                    alert(data.msg);
                                                                 
                                                              },
                                                               error: function (xhr, ajaxOptions, thrownError) {
                                                                  alert(xhr.status);
                                                                  alert(thrownError);
                                                                  //waitingDialog.hide();
                                                                   waitingDialog.hide(); 
                                                                }

                                                          });
                                                
                                               }

                                        }
                                 });
                             
                            
                                

                                    
                                
                            
                            
                        
                         
                                                   
                          
                                
                                    
                                        
                                            Add Participant
                                        
                                        
                                           

                                             

                                               
                                            
                                                   
                                                       Search Parameter
                                                       
                                                           
                                                               
                                                                   
                                                                        By Username
                                                                   
                                                                   
                                                                        By Group name
                                                                   

                                                               

                                                             

                                                       
                                                         
                                                    

                                               
                                            
                                                Message
                                                
                                                     Suppress notifications to Participant.
                                                     Suppress notifications to Me.
                                                
                                            

                                               
                                                   
                                                       Participant*
                                                       

                                                            
                                                       
                                                   
                                                      
                                                   
                                                          $('#participant').bootcomplete({
                                                          url:'/PGCTechnologyServices/course-enrollments/getusers',
                                                          'idField' :true,
                                                          'minLength':4,
                                                           callback: function() {
                                                               
                                                                $(&quot;#partici_name&quot;).val($(&quot;#participant&quot;).val().replace(/\s*\(.*?\)\s*/g, ''));
                                                               
                                                               }

                                                          });
                                                   
                                              
                                                
                                                   
                                                       Group Name*
                                                       

                                                            
                                                       
                                                   
                                                      
                                                   
                                                          $('.groupname').bootcomplete({
                                                          url:'/PGCTechnologyServices/course-manager/getclassgroups',
                                                          'idField' :true,
                                                          'minLength':4,
                                                           callback: function() {
                                                               }

                                                          });
                                                   
                                                 
                                            
                                                
                                                Register
                                            
                                        
                                        
                                        
                                        
                                      
                                      

                                       var arrayClients;
                                       var counter=0;
                                       $(&quot;[name='search_param']&quot;).click(function(){

                                           if($(this).val()=='0'){

                                                $(&quot;.byname&quot;).show();
                                                $(&quot;.bygroup&quot;).hide();

                                           }
                                           else{

                                               $(&quot;.byname&quot;).hide();
                                               $(&quot;.bygroup&quot;).show();

                                           }


                                       });


                                       var ascrole='';
                                                                                  ascrole=0;
                                           
                                       $.validator.setDefaults({ ignore: '' });

                                            $.validator.addMethod(&quot;checkclassfull&quot;, function(value, element) {

                                               if($('.coursefull').css('display') == 'none'){

                                                   return true;
                                               }

                                               if( $(&quot;input:radio[name='cgCourseFull']&quot;).is(&quot;:checked&quot;)) {

                                                      return true;
                                                  }    
                                                  else {

                                                           return false;

                                                  }


                                             }, &quot;Please select one option &quot;);


                                            $.validator.addMethod(&quot;chkRsvSeat_&quot;, function(value, element) {

                                               if($('.ReservedSeatsFieldset').css('display') == 'none'){

                                                return true;

                                               }

                                               if($(&quot;input:checkbox[name='chkRsvSeat']&quot;).is(&quot;:checked&quot;)) {

                                                      return true;
                                                  }    
                                                  else {

                                                      return true;

                                                  }


                                             }, &quot;Please select option &quot;);


                                          $('#frm_add_participant').validate({


                                                rules: {

                                                         participant_id: {
                                                               required: {
                                                                   depends: function(element){

                                                                       if(!$('input[name=search_param]').length){

                                                                           return true;
                                                                       }
                                                                       else{
                                                                           return $('input[name=search_param]:checked').val()==&quot;0&quot;  
                                                                       }
                                                                   }
                                                               }
                                                           },
                                                            groupname_id: {
                                                               required: {
                                                                   depends: function(element){

                                                                       return $('input[name=search_param]:checked').val()==&quot;2&quot;
                                                                   }
                                                               }
                                                           }


                                                       /*,
                                                        cgCourseFull_hidden:{
                                                            checkclassfull:true
                                                        },
                                                        chkRsvSeat_hidden:{
                                                            chkRsvSeat_:true
                                                        }*/

                                                },
                                                 errorPlacement: function(error, element) {
                                                        error.appendTo( element.closest(&quot;.elementclass&quot;).next());
                                                  }


                                            });

                                            
                                             $('#btnaddParticipant').click(function () {

                                               $('#frm_add_participant').validate();
                                                   if ($(&quot;#frm_add_participant&quot;).valid()) {

                                                       var rgSearchOption=0;
                                                        var Action='';
                                                        OR_CONFLICT=3;
                                                        OR_EVAL=4;
                                                        subClientID = $(&quot;#participant_id&quot;).val();
                                                        selectedClass = 3267;
                                                        selectedGroup=$(&quot;#groupname&quot;).val();
                                                        datastring={};
                                                        var iscomplete=true;

                                                        if($('#search_param_group').length >0){

                                                            if($('#search_param_group').is(':checked')){

                                                                rgSearchOption=2;
                                                            } 
                                                        }
                                                            
                                                        if(rgSearchOption!=2){
                                                            
                                                           arrayClients=''; 
                                                           counter=0;
                                                           
                                                           $(&quot;#reguser&quot;).remove();
                                                           $.when(
                                                                      $.ajax({
                                                                          
                                                                             type: &quot;POST&quot;,
                                                                             cache: false,
                                                                             timeout:600000,
                                                                             url: &quot;/PGCTechnologyServices/course-enrollments/setactioninsession?action=&quot;,
                                                                             success: function(data) {
                                                                                  
                                                                                },
                                                                                 error: function (xhr, ajaxOptions, thrownError) {
                                                                                        //alert(xhr.status);
                                                                                        //alert(thrownError);
                                                                                        //waitingDialog.hide();
                                                                                        waitingDialog.hide();
                                                                                       $('.ajax_instr_loader').hide();
                                                                                       $(&quot;#hide_ajax_loader&quot;).val(&quot;false&quot;);
                                                                                  }

                                                                        }),
                                                                       
                                                                          $.ajax({
                                                                             type: &quot;POST&quot;,
                                                                             cache: false,
                                                                             timeout:600000,
                                                                             url: &quot;/PGCTechnologyServices/course-enrollments/isclassconflict?classid=&quot;+selectedClass+'&amp;ClientId='+subClientID,
                                                                             data: datastring,
                                                                             beforeSend:function() {
                                             
                                                                                    //waitingDialog.hide();
                                                                                    //$('.ajax_instr_loader').hide();
                                                                                    //waitingDialog.show('Loading....');
                                                                              },
                                                                              complete: function() {
                                             
                                                                                    //waitingDialog.hide();
                                                                                    //$('.ajax_instr_loader').hide();
                                                                                },        
                                                                              success: function(data) {

                                                                                    data=jQuery.parseJSON(data);

                                                                                    //'SDS.04


                                                                                       if(data.result=='1') {
                                                                                         var Msg = &quot;This course conflicts with one or more other open enrollments for '&quot;+$(&quot;#participant&quot;).val().replace(/\s*\(.*?\)\s*/g, '')+&quot;'&quot;;
                                                                                         alert(Msg);

                                                                                          if(&quot;0&quot;== '1' ){


                                                                                            if(ascrole == &quot;0&quot; || ascrole == &quot;1&quot;){
                                                                                                var ms=&quot; Would you like to continue registering '&quot;+$(&quot;#participant&quot;).val().replace(/\s*\(.*?\)\s*/g, '')+&quot;'?&quot;; 
                                                                                                r=confirm(ms);

                                                                                                if(r!=true){
                                                                                                  iscomplete=false;    
                                                                                                 }
                                                                                                 else{
                                                                                                     
                                                                                                    
                                                                                                        Action+='|'+'3'+'|';

                                                                                                    
                                                                                                     
                                                                                                 }
                                                                                                 

                                                                                            }
                                                                                            else{

                                                                                                  iscomplete=false;
                                                                                            }
                                                                                        }    
                                                                                         else{

                                                                                              var Msg = &quot;Would you like to continue?&quot;;

                                                                                               r=confirm(Msg);
                                                                                               if(r!=true){
                                                                                                  iscomplete=false;   
                                                                                               }


                                                                                              //Action += &quot;|&quot; + $.trim(OR_CONFLICT) + &quot;|&quot;;

                                                                                          }

                                                                                      }




                                                                               },
                                                                                 error: function (xhr, ajaxOptions, thrownError) {
                                                                                        //alert(xhr.status);
                                                                                        //alert(thrownError);
                                                                                        //waitingDialog.hide();
                                                                                        waitingDialog.hide();
                                                                                       $('.ajax_instr_loader').hide();
                                                                                       $(&quot;#hide_ajax_loader&quot;).val(&quot;false&quot;);
                                                                                  }

                                                                            })

                                                                                   
                                                                     
                                                                
                                                             ).then(function() {   

                                                               if($(&quot;input:checkbox[name='chkRsvSeat']&quot;).is(&quot;:checked&quot;)) {

                                                                  var x = new Date('1900-01-01 00:00:00.000');
                                                                  var y = new Date('2020-07-28');
                                                                  if(x &lt; y){

                                                                     $msg = &quot;No more reserved seats available. Reserve period has passed.&quot; ;
                                                                     alert($msg);
                                                                     iscomplete=false;   
                                                                  }

                                                               }

                                                               contRegisration(iscomplete,selectedClass,subClientID,Action);

                                                            });
                                                         }
                                                         else{
                                                         
                                                                $(&quot;#warnings&quot;).val('');
                                                                $('#warnings_ul > li').remove();
                                                                cgCourseFull_=-1;
                                                                if($('input[name=cgCourseFull]').length>0){
                                                                
                                                                   cgCourseFull_=$('input[name=cgCourseFull]:checked').val(); 
                                                                    
                                                                }
                                                                
                                                                is_ovveride=false;
                                                                cgCourseFull_group=-1;
                                                                
                                                                 $.ajax({
                                                                           type: &quot;POST&quot;,
                                                                           cache: false,
                                                                           timeout:6000000,
                                                                           url: &quot;/PGCTechnologyServices/course-enrollments/registerforclassgroup?class_id=&quot;+selectedClass+'&amp;groupname='+selectedGroup+'&amp;cgCourseFull_='+cgCourseFull_+'&amp;confirm=0',
                                                                           beforeSend:function() {
                                             
                                                                                   // waitingDialog.hide();
                                                                                   // $('.ajax_instr_loader').hide();
                                                                                   // waitingDialog.show('Loading....');
                                                                              },
                                                                              complete: function() {
                                             
                                                                                    //waitingDialog.hide();
                                                                                    //$('.ajax_instr_loader').hide();
                                                                             }, 
                                                                            success: function(data) {
                                                                                  data=jQuery.parseJSON(data);
                                                                                  if(data.error==&quot;true&quot;){
                                                                                      
                                                                                     waitingDialog.hide(); 
                                                                                     alert(data.msg);  
                                                                                     return false;
                                                                                      
                                                                                  }
                                                                                  else if(data.warn!=&quot;&quot;){
                                                                                      
                                                                                      
                                                                                        r=confirm(data.warn);
                                                                                        if(r!=true){
                                                                                          
                                                                                          waitingDialog.hide();
                                                                                          return false;
                                                                                        }
                                                                                        else{
                                                                                            
                                                                                            is_ovveride=true;
                                                                                            cgCourseFull_group=2;
                                                                                        }
                                                                                      
                                                                                  }
                                                                                  
                                                                                  
                                                                                  arrayClients = data.data.split(',');
                                                                                  groupmem=arrayClients;
                                                                                  if(arrayClients.length > 0){
                                                                                   
                                                                                      if(count&lt;=arrayClients.length){
                                                                                      
                                                                                           
                                                                                        refreshIntervalId=setInterval(function () {
                                                                                            
                                                                                            $(&quot;#hide_ajax_loader&quot;).val(&quot;true&quot;);
                                                                                            pleaseWaitDiv2.modal('show');
                                                                                            var flg=false;   
                                                                                            if(count>groupmem.length-1){
                                                                                               
                                                                                               if($(&quot;#reg_status&quot;).val()!='start'){
                                                                                                        
                                                                                                        if($.trim($(&quot;#warnings&quot;).val())!=''){
                                                                                                            
                                                                                                           //alert(&quot;Group registration process completed with below warnings.&quot;+$(&quot;#warnings&quot;).val()); 
                                                                                                                     
                                                                                                                     var magnificPopup = $.magnificPopup.instance; 
                                                                                                                     magnificPopup.close(); 
                                                                                                                
                                                                                                                
                                                                                                                       $('#warnings_ul').append($(&quot;#warnings&quot;).val());
                                                                                                                       OpenWarning();
                                                                                                                        pleaseWaitDiv2.modal('hide');
                                                                                                                         $(&quot;#hide_ajax_loader&quot;).val(&quot;false&quot;);
                                                                                                                       
                                                                                                                       
                                                                                                                    
                                                                                                            }
                                                                                                            else{
                                                                                                                
                                                                                                                var magnificPopup = $.magnificPopup.instance; 
                                                                                                                magnificPopup.close(); 
                                                                                                                 alert(&quot;Group registration process completed&quot;);
                                                                                                                 $(&quot;#warnings&quot;).val('');
                                                                                                                 $('#warnings_ul > li').remove();
                                                                                                                 pleaseWaitDiv2.modal('hide');
                                                                                                                 $(&quot;#hide_ajax_loader&quot;).val(&quot;false&quot;);
                                                                                                                 flg=true;
                                                                                                                 clearInterval(refreshIntervalId);   
                                                                                                            }
                                                                                                        
                                                                                                        $(&quot;#reguser&quot;).remove();
                                                                                                        clearInterval(refreshIntervalId);   

                                                                                                          // $.pjax.reload({container:'#new-model'});

                                                                                                          //var magnificPopup = $.magnificPopup.instance; 
                                                                                                           //magnificPopup.close(); 
                                                                                                           $(&quot;#groupname&quot;).val('');
                                                                                                           $(&quot;#groupname_id&quot;).val('');
                                                                                                           
                                                                                                           $(&quot;#participant&quot;).val('');
                                                                                                           $(&quot;#partici_name&quot;).val('');
                                                                                                           $.pjax.reload({container:'#new-model',timeout:false}).done(function() { 

                                                                                                               $.pjax.reload({container:'#new-model-price',timeout:false}).done(function() { 

                                                                                                                   /* $.magnificPopup.open({
                                                                                                                        items: {
                                                                                                                            src: '#add-participant' 
                                                                                                                        },
                                                                                                                        type: 'inline',
                                                                                                                        closeOnBgClick:false,
                                                                                                                        enableEscapeKey:false
                                                                                                                    });*/

                                                                                                                     $.pjax.reload({container:'#new-model-participant',timeout:false}).done(function() { 

                                                                                                                            $.pjax.reload({container:'#new-model-certificate',timeout:false}).done(function() { 

                                                                                                                                    $.pjax.reload({container:'#new-model-markall',timeout:false}).done(function() { 

                                                                                                                                         $.pjax.reload({container:'#new-model-waitlist',timeout:false}).done(function() { 
                                                                                                                                             
                                                                                                                                             $.pjax.reload({container:'#new-model-request',timeout:false}).done(function() { 
                                                                                                                                                 $.pjax.reload({container:'#eval_model',timeout:false}).done(function() { 
                                                                                                                                                 
                                                                                                                                                            if($(&quot;#reguser&quot;).length>0){
                                                                                                                                                              $(&quot;#reguser&quot;).remove();  
                                                                                                                                                             }

                                                                                                                                                              $(&quot;#checked_participants_request&quot;).val('');
                                                                                                                                                              $(&quot;#is_checked_all_request&quot;).val('');
                                                                                                                                                              $(&quot;#unchecked_participants_request&quot;).val('');
                                                                                                                                                              $(&quot;#declined_reason_main&quot;).val('');
                                                                                                                                                              if(flg){

                                                                                                                                                                   $.magnificPopup.open({
                                                                                                                                                                      items: {
                                                                                                                                                                          src: '#add-participant' 
                                                                                                                                                                      },
                                                                                                                                                                      type: 'inline',
                                                                                                                                                                      closeOnBgClick:false,
                                                                                                                                                                      enableEscapeKey:false
                                                                                                                                                                  });
                                                                                                                                                              }

                                                                                                                                                            return false;
                                                                                                                                                      })      
                                                                                                                                                })
                                                                                                                                               
                                                                                                                                           })

                                                                                                                                    })
                                                                                                                             })
                                                                                                                       })


                                                                                                               })


                                                                                                           })
                                                                                                 
                                                                                                    } 
                                                                                             }
                                                                                             else{
                                                                                                
                                                                                                if($(&quot;#reg_status&quot;).val()!='start'){
                                                                                                    
                                                                                                    var participant=0;
                                                                                                    var partici_name='';
                                                                                                   
                                                                                                    var val_=groupmem[count];
                                                                                                    val_arr=val_.split('|');
                                                                                                    
                                                                                                    participant=$.trim(val_arr[0]);
                                                                                                    partici_name=$.trim(val_arr[1]);
                                                                                                    
                                                                                                    $(&quot;#participant&quot;).val(participant);
                                                                                                    $(&quot;#partici_name&quot;).val(partici_name);
                                                                                                    
                                                                                                    
                                                                                                    $(&quot;#reg_status&quot;).val('');
                                                                                                    contRegisration_groupmember(arrayClients,participant,3267,is_ovveride,cgCourseFull_group);
                                                                                                    count++;  
                                                                                                    $(&quot;#reguser&quot;).remove();
                                                                                                    //$(&quot;#pleaseWaitDialog&quot;).find('h3').after('&lt;div id=&quot;reguser&quot; style=&quot;margin-top:15px&quot;>Registering '+count+' of '+groupmem.length +' users from '+selectedGroup+' group..&lt;/div>');
                                                                                                     $('#count_').text(count); 
                                                                                                     $('#total_').text(groupmem.length); 
                                                                                                     $('#group_name__').text($(&quot;#groupname&quot;).val().toLowerCase()+&quot; group&quot;); 
                                                                                                    
                                                                                                }
                                                                                           } 
                                                                                           
                                                                                        }, 500);

                                                                                           
                                                                                           
                                                                                       
                                                                                       }
                                                                                       
                                                                                       
                                                                                   }
                                                                                   

                                                                             },
                                                                               error: function (xhr, ajaxOptions, thrownError) {
                                                                                   
                                                                                    //alert(xhr.status);
                                                                                    //alert(thrownError);
                                                                                   //waitingDialog.hide();
                                                                                    waitingDialog.hide();
                                                                                    $('.ajax_instr_loader').hide();
                                                                                     $(&quot;#hide_ajax_loader&quot;).val(&quot;false&quot;);

                                                                                }

                                                                          })
                                                         
                                                         
                                                         }


                                                  }


                                            });    


                                           
                                          function contRegisration_groupmember(returnval,clientId,class_id,is_ovveride,cgCourseFull_group){
                                         
                                                iscomplete=true;
                                                selectedClass=class_id;
                                                subClientID=clientId;
                                                datastring={};
                                                var Action='';
                                                $(&quot;#reg_status&quot;).val('start');
                                                $.when(
                                                          $.ajax({
                                                                          
                                                                type: &quot;POST&quot;,
                                                                cache: false,
                                                                timeout:600000,
                                                                url: &quot;/PGCTechnologyServices/course-enrollments/setactioninsession?action=&quot;,
                                                                success: function(data) {

                                                                   },
                                                                    error: function (xhr, ajaxOptions, thrownError) {
                                                                           //alert(xhr.status);
                                                                           //alert(thrownError);
                                                                           //waitingDialog.hide();
                                                                           waitingDialog.hide();
                                                                          $('.ajax_instr_loader').hide();
                                                                          $(&quot;#hide_ajax_loader&quot;).val(&quot;false&quot;);
                                                                     }

                                                           }),   
                                                          
                                                             $.ajax({
                                                                type: &quot;POST&quot;,
                                                                cache: false,
                                                                timeout:600000,
                                                                url: &quot;/PGCTechnologyServices/course-enrollments/isclassconflict?classid=&quot;+selectedClass+'&amp;ClientId='+subClientID+'&amp;is_ovveride='+is_ovveride+'&amp;cgCourseFull_group='+cgCourseFull_group,
                                                                data: datastring,
                                                                beforeSend:function() {

                                                                       //waitingDialog.hide();
                                                                       //$('.ajax_instr_loader').hide();
                                                                       //waitingDialog.show('Loading....');
                                                                 },
                                                                 complete: function() {

                                                                       //waitingDialog.hide();
                                                                       //$('.ajax_instr_loader').hide();
                                                                   },        
                                                                 success: function(data) {

                                                                       data=jQuery.parseJSON(data);

                                                                       //'SDS.04


                                                                          if(data.result=='1') {
                                                                            var Msg = &quot;This course conflicts with one or more other open enrollments for '&quot;+$(&quot;#partici_name&quot;).val()+&quot;'&quot;;
                                                                            alert(Msg);

                                                                             if(&quot;0&quot;== '1' ){


                                                                               if(ascrole == &quot;0&quot; || ascrole == &quot;1&quot;){
                                                                                   var ms=&quot; Would you like to continue registering '&quot;+$(&quot;#partici_name&quot;).val()+&quot;'?&quot;; 
                                                                                   r=confirm(ms);

                                                                                   if(r!=true){
                                                                                     iscomplete=false;    
                                                                                    }
                                                                                    else{
                                                                                        
                                                                                        Action+='|'+'3'+'|';
                                                                                    }

                                                                               }
                                                                               else{

                                                                                     iscomplete=false;
                                                                               }
                                                                           }    
                                                                            else{

                                                                                 var Msg = &quot;Would you like to continue?&quot;;

                                                                                  r=confirm(Msg);
                                                                                  if(r!=true){
                                                                                     iscomplete=false;   
                                                                                  }


                                                                                 //Action += &quot;|&quot; + $.trim(OR_CONFLICT) + &quot;|&quot;;

                                                                             }

                                                                         }




                                                                  },
                                                                    error: function (xhr, ajaxOptions, thrownError) {
                                                                           //alert(xhr.status);
                                                                           //alert(thrownError);
                                                                           //waitingDialog.hide();
                                                                           waitingDialog.hide();
                                                                          $('.ajax_instr_loader').hide();
                                                                          $(&quot;#hide_ajax_loader&quot;).val(&quot;false&quot;);
                                                                          
                                                                     }

                                                               })

                                                                      
                                                        
                                                   
                                                ).then(function() {   

                                                  if($(&quot;input:checkbox[name='chkRsvSeat']&quot;).is(&quot;:checked&quot;)) {

                                                     var x = new Date('1900-01-01 00:00:00.000');
                                                     var y = new Date('2020-07-28');
                                                     if(x &lt; y){

                                                        $msg = &quot;No more reserved seats available. Reserve period has passed.&quot; ;
                                                        alert($msg);
                                                        iscomplete=false;   
                                                     }

                                                  }

                                                  contRegisration_group_mem(iscomplete,selectedClass,subClientID,is_ovveride,cgCourseFull_group,Action);

                                               }); 
                                             }                 
                                             
                                          function contRegisration_group_mem(returnval,selectedClass,subClientID,is_ovveride,cgCourseFull_group,act){

                                            
                                               if(returnval==true){


                                                  var myform = $('#frm_add_participant');
                                                  var serialized = myform.serialize();

                                                  var datastring = serialized;
                                                  $.ajax({
                                                           type: &quot;POST&quot;,
                                                           cache: false,
                                                           timeout:600000,
                                                           url: &quot;/PGCTechnologyServices/course-enrollments/registerforclass?class_id=&quot;+selectedClass+'&amp;clientId='+subClientID+'&amp;RegisterAsFlag=1'+'&amp;is_ovveride='+is_ovveride+'&amp;cgCourseFull_group='+cgCourseFull_group+'&amp;act='+act,
                                                           data: datastring,
                                                           beforeSend:function() {
                                             
                                                                //waitingDialog.hide();
                                                                //$('.ajax_instr_loader').hide();
                                                                //waitingDialog.show('Loading....');
                                                          },
                                                          complete: function() {

                                                                //waitingDialog.hide();
                                                                //$('.ajax_instr_loader').hide();
                                                            }, 
                                                           success: function(data) {

                                                                data=jQuery.parseJSON(data);
                                                                
                                                                if(data.error==&quot;true&quot;){

                                                                   if(data.hasOwnProperty('msg')){
                                                                        
                                                                        $(&quot;#warnings&quot;).val($(&quot;#warnings&quot;).val()+&quot;&lt;li>&quot;+data.msg+&quot;&lt;/li>&quot;);
                                                                         //alert(data.msg);
                                                                         $(&quot;#reg_status&quot;).val('done');
                                                                         
                                                                         if($(&quot;#reguser&quot;).length>0){
                                                                                $(&quot;#reguser&quot;).remove();  
                                                                          }
                                                                         
                                                                         if(data.msg.indexOf('process your request [pkey]') > -1 || data.msg.indexOf('license is expired') > -1  || data.msg.indexOf('software is unlicensed') > -1 || data.msg.indexOf('limit exceeded') > -1 || data.msg.indexOf('Enrollment group limits have been reached') >-1 ){
                                                                             
                                                                              alert(data.msg);
                                                                              var magnificPopup = $.magnificPopup.instance; 
                                                                              magnificPopup.close();
                                                                              
                                                                               $(&quot;#warnings&quot;).val('');
                                                                                $('#warnings_ul > li').remove();
                                                                                pleaseWaitDiv2.modal('hide');
                                                                                $(&quot;#hide_ajax_loader&quot;).val(&quot;false&quot;);
                                                                                flg=true;
                                                                                clearInterval(refreshIntervalId);   
                                                                                
                                                                               $.pjax.reload({container:'#new-model',timeout:false}).done(function() { 

                                                                                        
                                                                                          $.pjax.reload({container:'#new-model-price',timeout:false}).done(function() { 
                                                                                          
                                                                                            
                                                                                                $.pjax.reload({container:'#new-model-participant',timeout:false}).done(function() { 
                                                                                                        
                                                                                                       $.pjax.reload({container:'#new-model-certificate',timeout:false}).done(function() { 

                                                                                                            $.pjax.reload({container:'#new-model-markall',timeout:false}).done(function() { 
                                                                                                                    
                                                                                                                    $.pjax.reload({container:'#new-model-waitlist',timeout:false}).done(function() { 
                                                                                                                        
                                                                                                                        $.pjax.reload({container:'#new-model-request',timeout:false}).done(function() { 
                                                                                                                              
                                                                                                                            $.pjax.reload({container:'#eval_model',timeout:false}).done(function() {
                                                                                                                                
                                                                                                                                    $(&quot;#reg_status&quot;).val('done'); 
                                                                                                                                      if($(&quot;#reguser&quot;).length>0){
                                                                                                                                          $(&quot;#reguser&quot;).remove();  
                                                                                                                                       }
                                                                                                                                    $(&quot;#checked_participants_request&quot;).val('');
                                                                                                                                    $(&quot;#is_checked_all_request&quot;).val('');
                                                                                                                                    $(&quot;#unchecked_participants_request&quot;).val('');
                                                                                                                                    $(&quot;#declined_reason_main&quot;).val('');  
                                                                                                                                  })
                                                                                                                              })     
                                                                                            
                                                                                                                    })
                                                                                                             })
                                                                                                        })
                                                                                                })
                                                                                                
                                                                                              
                                                                                                
                                                                                               
                                                                                                
                                                                                          })

                                                                                })
                                                                              clearInterval(refreshIntervalId);
                                                                              
                                                                              
                                                                              
                                                                         }
                                                                   }
                                                                   else if(data.hasOwnProperty('message')){

                                                                        //alert(data.message);
                                                                        
                                                                         $(&quot;#warnings&quot;).val($(&quot;#warnings&quot;).val()+&quot;&lt;li>&quot;+data.message+&quot;&lt;/li>&quot;);
                                                                         if( data.message.indexOf('process your request [pkey]') > -1 || data.message.indexOf('license is expired') > -1 || data.message.indexOf('software is unlicensed') > -1 || data.message.indexOf('limit exceeded') > -1 || data.message.indexOf('Enrollment group limits have been reached') >-1 ){
                                                                             
                                                                              alert(data.message);
                                                                              var magnificPopup = $.magnificPopup.instance; 
                                                                              magnificPopup.close();
                                                                               $(&quot;#warnings&quot;).val('');
                                                                                $('#warnings_ul > li').remove();
                                                                                pleaseWaitDiv2.modal('hide');
                                                                                $(&quot;#hide_ajax_loader&quot;).val(&quot;false&quot;);
                                                                                flg=true;
                                                                                clearInterval(refreshIntervalId);   

                                                                              
                                                                                $.pjax.reload({container:'#new-model',timeout:false}).done(function() { 

                                                                                        
                                                                                          $.pjax.reload({container:'#new-model-price',timeout:false}).done(function() { 
                                                                                          
                                                                                            
                                                                                         
                                                                                                $.pjax.reload({container:'#new-model-participant',timeout:false}).done(function() { 
                                                                                                        
                                                                                                       $.pjax.reload({container:'#new-model-certificate',timeout:false}).done(function() { 

                                                                                                            $.pjax.reload({container:'#new-model-markall',timeout:false}).done(function() { 
                                                                                                                    
                                                                                                                    $.pjax.reload({container:'#new-model-waitlist',timeout:false}).done(function() { 
                                                                                                                        
                                                                                                                        $.pjax.reload({container:'#new-model-request',timeout:false}).done(function() { 
                                                                                                                        
                                                                                                                                $.pjax.reload({container:'#eval_model',timeout:false}).done(function() { 
                                                                                                                                                 
                                                                                                                                    $(&quot;#reg_status&quot;).val('done'); 
                                                                                                                                      if($(&quot;#reguser&quot;).length>0){
                                                                                                                                          $(&quot;#reguser&quot;).remove();  
                                                                                                                                       }
                                                                                                                                    $(&quot;#checked_participants_request&quot;).val('');
                                                                                                                                    $(&quot;#is_checked_all_request&quot;).val('');
                                                                                                                                    $(&quot;#unchecked_participants_request&quot;).val('');
                                                                                                                                    $(&quot;#declined_reason_main&quot;).val(''); 
                                                                                                                                    
                                                                                                                                })    
                                                                                                                          })
                                                                                                                        
                                                                                            
                                                                                                                    })
                                                                                                             })
                                                                                                        })
                                                                                                })
                                                                                                
                                                                                              
                                                                                                
                                                                                               
                                                                                                
                                                                                          })

                                                                                })
                                                                              clearInterval(refreshIntervalId);
                                                                              
                                                                              
                                                                            
                                                                              
                                                                         }
                                                                         
                                                                        $(&quot;#reg_status&quot;).val('done');
                                                                        if($(&quot;#reguser&quot;).length>0){
                                                                            $(&quot;#reguser&quot;).remove();  
                                                                         }

                                                                   }
                                                                   
                                                                   

                                                                }
                                                                else{

                                                                     if(data.error==&quot;false&quot;){

                                                                         if(data.message!='first_part_done'){

                                                                            
                                                                               $(&quot;#reg_status&quot;).val('done'); 
                                                                               if($(&quot;#reguser&quot;).length>0){
                                                                                    $(&quot;#reguser&quot;).remove();  
                                                                                 }
                                                                              
                                                                         }
                                                                         else{

                                                                                $.ajax({
                                                                                           type: &quot;POST&quot;,
                                                                                           cache: false,
                                                                                           timeout:600000,
                                                                                           url: &quot;/PGCTechnologyServices/course-enrollments/registerforclasspart2?class_id=&quot;+selectedClass+'&amp;clientId='+subClientID+'&amp;RegisterAsFlag=1'+'&amp;is_ovveride='+is_ovveride+'&amp;cgCourseFull_group='+cgCourseFull_group,
                                                                                           data: datastring,
                                                                                           beforeSend:function() {
                                             
                                                                                                //waitingDialog.hide();
                                                                                                //$('.ajax_instr_loader').hide();
                                                                                                //waitingDialog.show('Loading....');
                                                                                          },
                                                                                          complete: function() {

                                                                                                //waitingDialog.hide();
                                                                                                //$('.ajax_instr_loader').hide();
                                                                                            }, 
                                                                                           success: function(data) {
                                                                                               
                                                                                               if($.trim(data)!=''){

                                                                                                   data=jQuery.parseJSON(data);

                                                                                                    if(data.hasOwnProperty('msg')){
                                                                                                            
                                                                                                           $(&quot;#warnings&quot;).val($(&quot;#warnings&quot;).val()+&quot;&lt;li>&quot;+data.msg+&quot;&lt;/li>&quot;); 
                                                                                                           //alert(data.msg);
                                                                                                     }
                                                                                                     else if(data.hasOwnProperty('message')){

                                                                                                         $(&quot;#warnings&quot;).val($(&quot;#warnings&quot;).val()+&quot;&lt;li>&quot;+data.message+&quot;&lt;/li>&quot;); 
                                                                                                          //alert(data.message);

                                                                                                       }

                                                                                                   if(data.hasOwnProperty('message_confirm')){

                                                                                                       var r= confirm(data.message_confirm);

                                                                                                       if (r == true) {


                                                                                                          contRegisration3_group_mem(selectedClass,subClientID,is_ovveride,cgCourseFull_group);

                                                                                                       }
                                                                                                       else{

                                                                                                           $(&quot;#reg_status&quot;).val('done');  
                                                                                                           if($(&quot;#reguser&quot;).length>0){
                                                                                                                $(&quot;#reguser&quot;).remove();  
                                                                                                             }
                                                                                                           return false;
                                                                                                           
                                                                                                       }
                                                                                                   }
                                                                                                   else{

                                                                                                       contRegisration3_group_mem(selectedClass,subClientID,is_ovveride,cgCourseFull_group);
                                                                                                   }


                                                                                               }
                                                                                               else{

                                                                                                    contRegisration3_group_mem(selectedClass,subClientID,is_ovveride,cgCourseFull_group);

                                                                                               }



                                                                                           },
                                                                                           error: function (xhr, ajaxOptions, thrownError) {
                                                                                                
                                                                                                //alert(xhr.status);
                                                                                                //alert(thrownError);
                                                                                                //waitingDialog.hide();
                                                                                                waitingDialog.hide();
                                                                                                $('.ajax_instr_loader').hide();
                                                                                                $(&quot;#hide_ajax_loader&quot;).val(&quot;false&quot;);
                                                                                           
                                                                                            }

                                                                                      });


                                                                         }

                                                                     }


                                                                }

                                                             },
                                                               error: function (xhr, ajaxOptions, thrownError) {
                                                                    //alert(xhr.status);
                                                                    //alert(thrownError);
                                                                    //waitingDialog.hide();
                                                                    waitingDialog.hide();
                                                                    $('.ajax_instr_loader').hide();
                                                                    $(&quot;#hide_ajax_loader&quot;).val(&quot;false&quot;);
                                                                }

                                                          });


                                               }
                                               else{

                                                   $(&quot;#reg_status&quot;).val('done'); 
                                                   if($(&quot;#reguser&quot;).length>0){
                                                        $(&quot;#reguser&quot;).remove();  
                                                     }
                                                   return false;
                                               }
                                          }
                                          
                                          function contRegisration3_group_mem(selectedClass,subClientID,is_ovveride,cgCourseFull_group){


                                                 var myform = $('#frm_add_participant');
                                                 var serialized = myform.serialize();
                                                 var datastring = serialized;

                                                 $.ajax({
                                                           type: &quot;POST&quot;,
                                                           cache: false,
                                                           timeout:600000,
                                                           url: &quot;/PGCTechnologyServices/course-enrollments/registerforclasspart3?class_id=&quot;+selectedClass+'&amp;clientId='+subClientID+'&amp;RegisterAsFlag=1'+'&amp;is_ovveride='+is_ovveride+'&amp;cgCourseFull_group='+cgCourseFull_group,
                                                           data: datastring,
                                                           beforeSend:function() {
                                             
                                                                //waitingDialog.hide();
                                                                //$('.ajax_instr_loader').hide();
                                                                //waitingDialog.show('Loading....');
                                                          },
                                                          complete: function() {

                                                                //waitingDialog.hide();
                                                                //$('.ajax_instr_loader').hide();
                                                            }, 
                                                           success: function(data) {

                                                               if($.trim(data)!=''){

                                                                   data=jQuery.parseJSON(data);
                                                                   if(data.alert==&quot;true&quot;){


                                                                        if(data.hasOwnProperty('msg')){

                                                                               $(&quot;#warnings&quot;).val($(&quot;#warnings&quot;).val()+&quot;&lt;li>&quot;+data.msg+&quot;&lt;/li>&quot;);  
                                                                               //alert(data.msg);
                                                                         }
                                                                         else if(data.hasOwnProperty('message')){

                                                                              $(&quot;#warnings&quot;).val($(&quot;#warnings&quot;).val()+&quot;&lt;li>&quot;+data.message+&quot;&lt;/li>&quot;); 
                                                                              //alert(data.message);

                                                                           }
                                                                   }

                                                                   if(data.hasOwnProperty('message_confirm')){

                                                                       var r= confirm(data.message_confirm);

                                                                       if (r == true) {


                                                                          contRegisration4_group_mem(selectedClass,subClientID,is_ovveride,cgCourseFull_group);

                                                                       }
                                                                       else{

                                                                           $(&quot;#reg_status&quot;).val('done');  
                                                                           if($(&quot;#reguser&quot;).length>0){
                                                                                $(&quot;#reguser&quot;).remove();  
                                                                             }
                                                                           return false;
                                                                       }
                                                                   }
                                                                   else{

                                                                       contRegisration4_group_mem(selectedClass,subClientID,is_ovveride,cgCourseFull_group);
                                                                   }


                                                               }
                                                               else{

                                                                    contRegisration4_group_mem(selectedClass,subClientID,is_ovveride,cgCourseFull_group);

                                                               }



                                                           },
                                                           error: function (xhr, ajaxOptions, thrownError) {
                                                                //alert(xhr.status);
                                                                //alert(thrownError);
                                                                //waitingDialog.hide();
                                                                waitingDialog.hide();
                                                                $('.ajax_instr_loader').hide();
                                                                 $(&quot;#hide_ajax_loader&quot;).val(&quot;false&quot;);

                                                            }

                                                      });
                                             }
                                             
                                          function contRegisration3_group_mem(selectedClass,subClientID,is_ovveride,cgCourseFull_group){


                                                 var myform = $('#frm_add_participant');
                                                 var serialized = myform.serialize();
                                                 var datastring = serialized;

                                                 $.ajax({
                                                           type: &quot;POST&quot;,
                                                           cache: false,
                                                           timeout:600000,
                                                           url: &quot;/PGCTechnologyServices/course-enrollments/registerforclasspart3?class_id=&quot;+selectedClass+'&amp;clientId='+subClientID+'&amp;RegisterAsFlag=1'+'&amp;is_ovveride='+is_ovveride+'&amp;cgCourseFull_group='+cgCourseFull_group,
                                                           data: datastring,
                                                           beforeSend:function() {
                                             
                                                                //waitingDialog.hide();
                                                                //$('.ajax_instr_loader').hide();
                                                                //waitingDialog.show('Loading....');
                                                          },
                                                          complete: function() {

                                                                //waitingDialog.hide();
                                                                //$('.ajax_instr_loader').hide();
                                                            }, 
                                                           success: function(data) {

                                                               if($.trim(data)!=''){

                                                                   data=jQuery.parseJSON(data);
                                                                   if(data.alert==&quot;true&quot;){


                                                                        if(data.hasOwnProperty('msg')){
                                                                               $(&quot;#warnings&quot;).val($(&quot;#warnings&quot;).val()+&quot;&lt;li>&quot;+data.msg+&quot;&lt;/li>&quot;);  
                                                                               //alert(data.msg);
                                                                         }
                                                                         else if(data.hasOwnProperty('message')){

                                                                              //alert(data.message);
                                                                              $(&quot;#warnings&quot;).val($(&quot;#warnings&quot;).val()+&quot;&lt;li>&quot;+data.message+&quot;&lt;/li>&quot;); 

                                                                           }
                                                                   }

                                                                   if(data.hasOwnProperty('message_confirm')){

                                                                       var r= confirm(data.message_confirm);

                                                                       if (r == true) {


                                                                          contRegisration4_group_mem(selectedClass,subClientID,is_ovveride,cgCourseFull_group);

                                                                       }
                                                                       else{

                                                                           $(&quot;#reg_status&quot;).val('done');  
                                                                           if($(&quot;#reguser&quot;).length>0){
                                                                                $(&quot;#reguser&quot;).remove();  
                                                                             }
                                                                           return false;
                                                                       }
                                                                   }
                                                                   else{

                                                                       contRegisration4_group_mem(selectedClass,subClientID,is_ovveride,cgCourseFull_group);
                                                                   }


                                                               }
                                                               else{

                                                                    contRegisration4_group_mem(selectedClass,subClientID,is_ovveride,cgCourseFull_group);

                                                               }



                                                           },
                                                           error: function (xhr, ajaxOptions, thrownError) {
                                                                //alert(xhr.status);
                                                                //alert(thrownError);
                                                                //waitingDialog.hide();
                                                                waitingDialog.hide();
                                                                $('.ajax_instr_loader').hide();
                                                                 $(&quot;#hide_ajax_loader&quot;).val(&quot;false&quot;);

                                                            }

                                                      });
                                             }   
                                             
                                          function contRegisration4_group_mem(selectedClass,subClientID,is_ovveride,cgCourseFull_group){

                                              
                                                 var myform = $('#frm_add_participant');
                                                 var serialized = myform.serialize();
                                                 var datastring = serialized;

                                                 $.ajax({
                                                           type: &quot;POST&quot;,
                                                           cache: false,
                                                           timeout:600000,
                                                           url: &quot;/PGCTechnologyServices/course-enrollments/registerforclasspart4?class_id=&quot;+selectedClass+'&amp;clientId='+subClientID+'&amp;RegisterAsFlag=1'+'&amp;is_ovveride='+is_ovveride+'&amp;cgCourseFull_group='+cgCourseFull_group,
                                                           data: datastring,
                                                           beforeSend:function() {
                                             
                                                                //waitingDialog.hide();
                                                                //$('.ajax_instr_loader').hide();
                                                                //waitingDialog.show('Loading....');
                                                          },
                                                          complete: function() {

                                                                //waitingDialog.hide();
                                                                //$('.ajax_instr_loader').hide();
                                                            }, 
                                                           success: function(data) {
                                                                
                                                                 data=jQuery.parseJSON(data);  
                                                                 if(data.hasOwnProperty('confirm')){

                                                                       var r= confirm(data.message);

                                                                       if (r == true) {


                                                                          contRegisration5_group_mem(selectedClass,subClientID,is_ovveride,cgCourseFull_group);

                                                                       }
                                                                       else{

                                                                            $(&quot;#reg_status&quot;).val('done'); 
                                                                            if($(&quot;#reguser&quot;).length>0){
                                                                                $(&quot;#reguser&quot;).remove();  
                                                                             }
                                                                           return false;
                                                                       }

                                                                 }
                                                                 else if(data.hasOwnProperty('group_reg_cont')){
                                                                     
                                                                     contRegisration5_group_mem(selectedClass,subClientID,is_ovveride,cgCourseFull_group);
                                                                     
                                                                 }
                                                                 else{

                                                                     contRegisration5_group_mem(selectedClass,subClientID,is_ovveride,cgCourseFull_group);
                                                                 }


                                                           },
                                                           error: function (xhr, ajaxOptions, thrownError) {
                                                                //alert(xhr.status);
                                                                //alert(thrownError);
                                                                //waitingDialog.hide();
                                                                waitingDialog.hide();
                                                                $('.ajax_instr_loader').hide();
                                                                $(&quot;#hide_ajax_loader&quot;).val(&quot;false&quot;);

                                                            }

                                                      });

                                             }   
                                            
                                          function contRegisration5_group_mem(selectedClass,subClientID,is_ovveride,cgCourseFull_group){


                                                 var myform = $('#frm_add_participant');
                                                 var serialized = myform.serialize();
                                                 var datastring = serialized;

                                                 $.ajax({
                                                           type: &quot;POST&quot;,
                                                           timeout:6000000,
                                                           url: &quot;/PGCTechnologyServices/course-enrollments/registerforclasspart5?class_id=&quot;+selectedClass+'&amp;clientId='+subClientID+'&amp;RegisterAsFlag=1'+'&amp;is_ovveride='+is_ovveride+'&amp;cgCourseFull_group='+cgCourseFull_group,
                                                           data: datastring,
                                                           beforeSend:function() {

                                                                //waitingDialog.hide();
                                                                //$('.ajax_instr_loader').hide();
                                                                //waitingDialog.show('Loading....');
                                                          },
                                                          complete: function() {

                                                                //waitingDialog.hide();
                                                                //$('.ajax_instr_loader').hide();
                                                                
                                                                $(&quot;#reg_status&quot;).val('done'); 
                                                                if($(&quot;#reguser&quot;).length>0){
                                                                    $(&quot;#reguser&quot;).remove();  
                                                                 }
                                                                
                                                            }, 
                                                           success: function(data) {
                                                                  data=jQuery.parseJSON(data);  
                                                                  if(data.error==&quot;true&quot;){

                                                                       if(data.hasOwnProperty('msg')){

                                                                               //alert(data.msg);
                                                                               
                                                                               $(&quot;#warnings&quot;).val($(&quot;#warnings&quot;).val()+&quot;&lt;li>&quot;+data.msg+&quot;&lt;/li>&quot;); 
                                                                               
                                                                         }
                                                                         else if(data.hasOwnProperty('message')){

                                                                              //alert(data.message);
                                                                              $(&quot;#warnings&quot;).val($(&quot;#warnings&quot;).val()+&quot;&lt;li>&quot;+data.message+&quot;&lt;/li>&quot;); 
                                                                           }
                                                                           
                                                                            $(&quot;#reg_status&quot;).val('done'); 
                                                                            if($(&quot;#reguser&quot;).length>0){
                                                                                $(&quot;#reguser&quot;).remove();  
                                                                             }
                                                                  }
                                                                  else{
                                                                        /*
                                                                         if(data.hasOwnProperty('msg')){

                                                                               alert(data.msg);
                                                                         }
                                                                         else if(data.hasOwnProperty('message')){

                                                                              alert(data.message);

                                                                           }
                                                                         */  
                                                                           $(&quot;#reg_status&quot;).val('done'); 
                                                                           if($(&quot;#reguser&quot;).length>0){
                                                                            $(&quot;#reguser&quot;).remove();  
                                                                         }
                                                                  }


                                                          

                                                    
                                                       
                                                        },
                                                           error: function (xhr, ajaxOptions, thrownError) {
                                                                //alert(xhr.status);
                                                                //alert(thrownError);
                                                                //waitingDialog.hide();
                                                                waitingDialog.hide();
                                                                $('.ajax_instr_loader').hide();
                                                                $(&quot;#hide_ajax_loader&quot;).val(&quot;false&quot;);

                                                            }

                                                   });

                                           }
                                         
                                         
                                          function contRegisration(returnval,selectedClass,subClientID,act){


                                               if(returnval==true){


                                                  var myform = $('#frm_add_participant');
                                                  var serialized = myform.serialize();

                                                  var datastring = serialized;
                                                  $.ajax({
                                                           type: &quot;POST&quot;,
                                                           cache: false,
                                                           timeout:600000,
                                                           url: &quot;/PGCTechnologyServices/course-enrollments/registerforclass?class_id=&quot;+selectedClass+'&amp;clientId='+subClientID+'&amp;RegisterAsFlag=1&amp;act='+act,
                                                           data: datastring,
                                                           beforeSend:function() {
                                             
                                                                //waitingDialog.hide();
                                                                //$('.ajax_instr_loader').hide();
                                                                //waitingDialog.show('Loading....');
                                                          },
                                                          complete: function() {

                                                                //waitingDialog.hide();
                                                                //$('.ajax_instr_loader').hide();
                                                            }, 
                                                           success: function(data) {

                                                                data=jQuery.parseJSON(data);
                                                                
                                                                if(data.error==&quot;true&quot;){

                                                                   if(data.hasOwnProperty('msg')){

                                                                         alert(data.msg);
                                                                         $(&quot;#reg_status&quot;).val('done');
                                                                         if($(&quot;#reguser&quot;).length>0){
                                                                            $(&quot;#reguser&quot;).remove();  
                                                                         }
                                                                   }
                                                                   else if(data.hasOwnProperty('message')){

                                                                        alert(data.message);
                                                                        $(&quot;#reg_status&quot;).val('done');
                                                                        if($(&quot;#reguser&quot;).length>0){
                                                                            $(&quot;#reguser&quot;).remove();  
                                                                         }

                                                                   }


                                                                }
                                                                else{

                                                                     if(data.error==&quot;false&quot;){

                                                                         if(data.message!='first_part_done'){

                                                                            
                                                                             
                                                                              var magnificPopup = $.magnificPopup.instance; 
                                                                                magnificPopup.close(); 
                                                                                $(&quot;#participant&quot;).val('');
                                                                                $(&quot;#participant_id&quot;).val('');

                                                                                $.pjax.reload({container:'#new-model',timeout:false}).done(function() { 

                                                                                        
                                                                                          $.pjax.reload({container:'#new-model-price',timeout:false}).done(function() { 
                                                                                          
                                                                                             $.magnificPopup.open({
                                                                                                    items: {
                                                                                                        src: '#add-participant' 
                                                                                                    },
                                                                                                    type: 'inline',
                                                                                                    closeOnBgClick:false,
                                                                                                    enableEscapeKey:false
                                                                                                });
                                                                                         
                                                                                                $.pjax.reload({container:'#new-model-participant',timeout:false}).done(function() { 
                                                                                                        
                                                                                                       $.pjax.reload({container:'#new-model-certificate',timeout:false}).done(function() { 

                                                                                                            $.pjax.reload({container:'#new-model-markall',timeout:false}).done(function() { 
                                                                                                                    
                                                                                                                    $.pjax.reload({container:'#new-model-waitlist',timeout:false}).done(function() { 
                                                                                                                        
                                                                                                                        $.pjax.reload({container:'#new-model-request',timeout:false}).done(function() { 
                                                                                                                            
                                                                                                                            $.pjax.reload({container:'#eval_model',timeout:false}).done(function() { 
                                                                                                                                                 
                                                                                                                                    $(&quot;#reg_status&quot;).val('done'); 
                                                                                                                                      if($(&quot;#reguser&quot;).length>0){
                                                                                                                                          $(&quot;#reguser&quot;).remove();  
                                                                                                                                       }
                                                                                                                                    $(&quot;#checked_participants_request&quot;).val('');
                                                                                                                                    $(&quot;#is_checked_all_request&quot;).val('');
                                                                                                                                    $(&quot;#unchecked_participants_request&quot;).val('');
                                                                                                                                    $(&quot;#declined_reason_main&quot;).val('');  
                                                                                                                                    
                                                                                                                                    
                                                                                                                               })     
                                                                                                                          })
                                                                                                                        
                                                                                            
                                                                                                                    })
                                                                                                             })
                                                                                                        })
                                                                                                })
                                                                                                
                                                                                              
                                                                                                
                                                                                               
                                                                                                
                                                                                          })

                                                                                })
                                                                                
                                                                                 alert(data.message);
                                                                         }
                                                                         else{
                                                                             
                                                                                $.ajax({
                                                                                           type: &quot;POST&quot;,
                                                                                           cache: false,
                                                                                           timeout:600000,
                                                                                           url: &quot;/PGCTechnologyServices/course-enrollments/registerforclasspart2?class_id=&quot;+selectedClass+'&amp;clientId='+subClientID+'&amp;RegisterAsFlag=1',
                                                                                           data: datastring,
                                                                                           beforeSend:function() {
                                             
                                                                                                //waitingDialog.hide();
                                                                                                //$('.ajax_instr_loader').hide();
                                                                                                //waitingDialog.show('Loading....');
                                                                                          },
                                                                                          complete: function() {

                                                                                                //waitingDialog.hide();
                                                                                                //$('.ajax_instr_loader').hide();
                                                                                            }, 
                                                                                           success: function(data) {
                                                                                               
                                                                                               if($.trim(data)!=''){

                                                                                                   data=jQuery.parseJSON(data);

                                                                                                    if(data.hasOwnProperty('msg')){

                                                                                                           alert(data.msg);
                                                                                                     }
                                                                                                     else if(data.hasOwnProperty('message')){

                                                                                                          alert(data.message);

                                                                                                       }

                                                                                                   if(data.hasOwnProperty('message_confirm')){

                                                                                                       var r= confirm(data.message_confirm);

                                                                                                       if (r == true) {


                                                                                                          contRegisration3(selectedClass,subClientID);

                                                                                                       }
                                                                                                       else{

                                                                                                           $(&quot;#reg_status&quot;).val('done');  
                                                                                                           if($(&quot;#reguser&quot;).length>0){
                                                                                                                $(&quot;#reguser&quot;).remove();  
                                                                                                            }
                                                                                                           return false;
                                                                                                           
                                                                                                       }
                                                                                                   }
                                                                                                   else{

                                                                                                       contRegisration3(selectedClass,subClientID);
                                                                                                   }


                                                                                               }
                                                                                               else{

                                                                                                    contRegisration3(selectedClass,subClientID);

                                                                                               }



                                                                                           },
                                                                                           error: function (xhr, ajaxOptions, thrownError) {
                                                                                                
                                                                                                //alert(xhr.status);
                                                                                                //alert(thrownError);
                                                                                                //waitingDialog.hide();
                                                                                                waitingDialog.hide();
                                                                                                $('.ajax_instr_loader').hide();
                                                                                                $(&quot;#hide_ajax_loader&quot;).val(&quot;false&quot;);
                                                                                            }

                                                                                      });


                                                                         }

                                                                     }


                                                                }

                                                             },
                                                               error: function (xhr, ajaxOptions, thrownError) {
                                                                    //alert(xhr.status);
                                                                    //alert(thrownError);
                                                                    //waitingDialog.hide();
                                                                    waitingDialog.hide();
                                                                    $('.ajax_instr_loader').hide();
                                                                    $(&quot;#hide_ajax_loader&quot;).val(&quot;false&quot;);
                                                                }

                                                          });


                                               }
                                               else{

                                                   $(&quot;#reg_status&quot;).val('done'); 
                                                   if($(&quot;#reguser&quot;).length>0){
                                                        $(&quot;#reguser&quot;).remove();  
                                                   }
                                                   return false;
                                               }
                                            }

                                          function contRegisration3(selectedClass,subClientID){


                                                 var myform = $('#frm_add_participant');
                                                 var serialized = myform.serialize();
                                                 var datastring = serialized;

                                                 $.ajax({
                                                           type: &quot;POST&quot;,
                                                           cache: false,
                                                           timeout:600000,
                                                           url: &quot;/PGCTechnologyServices/course-enrollments/registerforclasspart3?class_id=&quot;+selectedClass+'&amp;clientId='+subClientID+'&amp;RegisterAsFlag=1',
                                                           data: datastring,
                                                           beforeSend:function() {
                                             
                                                                //waitingDialog.hide();
                                                                //$('.ajax_instr_loader').hide();
                                                                //waitingDialog.show('Loading....');
                                                          },
                                                          complete: function() {

                                                                //waitingDialog.hide();
                                                                //$('.ajax_instr_loader').hide();
                                                            }, 
                                                           success: function(data) {

                                                               if($.trim(data)!=''){

                                                                   data=jQuery.parseJSON(data);
                                                                   if(data.alert==&quot;true&quot;){


                                                                        if(data.hasOwnProperty('msg')){

                                                                               alert(data.msg);
                                                                         }
                                                                         else if(data.hasOwnProperty('message')){

                                                                              alert(data.message);

                                                                           }
                                                                   }

                                                                   if(data.hasOwnProperty('message_confirm')){

                                                                       var r= confirm(data.message_confirm);

                                                                       if (r == true) {


                                                                          contRegisration4(selectedClass,subClientID);

                                                                       }
                                                                       else{

                                                                           $(&quot;#reg_status&quot;).val('done');  
                                                                           if($(&quot;#reguser&quot;).length>0){
                                                                                $(&quot;#reguser&quot;).remove();  
                                                                             }
                                                                           return false;
                                                                       }
                                                                   }
                                                                   else{

                                                                       contRegisration4(selectedClass,subClientID);
                                                                   }


                                                               }
                                                               else{

                                                                    contRegisration4(selectedClass,subClientID);

                                                               }



                                                           },
                                                           error: function (xhr, ajaxOptions, thrownError) {
                                                                //alert(xhr.status);
                                                                //alert(thrownError);
                                                                //waitingDialog.hide();
                                                                waitingDialog.hide();
                                                                $('.ajax_instr_loader').hide();
                                                                $(&quot;#hide_ajax_loader&quot;).val(&quot;false&quot;);

                                                            }

                                                      });
                                             }
                                         
                                          function contRegisration4(selectedClass,subClientID){

                                              
                                                 var myform = $('#frm_add_participant');
                                                 var serialized = myform.serialize();
                                                 var datastring = serialized;

                                                 $.ajax({
                                                           type: &quot;POST&quot;,
                                                           cache: false,
                                                           timeout:600000,
                                                           url: &quot;/PGCTechnologyServices/course-enrollments/registerforclasspart4?class_id=&quot;+selectedClass+'&amp;clientId='+subClientID+'&amp;RegisterAsFlag=1',
                                                           data: datastring,
                                                           beforeSend:function() {
                                             
                                                                //waitingDialog.hide();
                                                                //$('.ajax_instr_loader').hide();
                                                                //waitingDialog.show('Loading....');
                                                          },
                                                          complete: function() {

                                                                //waitingDialog.hide();
                                                                //$('.ajax_instr_loader').hide();
                                                            }, 
                                                           success: function(data) {
                                                                
                                                                 data=jQuery.parseJSON(data);  
                                                                 if(data.hasOwnProperty('confirm')){

                                                                       var r= confirm(data.message);

                                                                       if (r == true) {


                                                                          contRegisration5(selectedClass,subClientID);

                                                                       }
                                                                       else{

                                                                            $(&quot;#reg_status&quot;).val('done'); 
                                                                            if($(&quot;#reguser&quot;).length>0){
                                                                                $(&quot;#reguser&quot;).remove();  
                                                                             }
                                                                           return false;
                                                                       }

                                                                 }
                                                                 else if(data.hasOwnProperty('group_reg_cont')){
                                                                     
                                                                     contRegisration5(selectedClass,subClientID);
                                                                     
                                                                 }
                                                                 else{

                                                                     contRegisration5(selectedClass,subClientID);
                                                                 }


                                                           },
                                                           error: function (xhr, ajaxOptions, thrownError) {
                                                                //alert(xhr.status);
                                                                //alert(thrownError);
                                                                //waitingDialog.hide();
                                                                waitingDialog.hide();
                                                                $('.ajax_instr_loader').hide();
                                                                $(&quot;#hide_ajax_loader&quot;).val(&quot;false&quot;);

                                                            }

                                                      });

                                             }
    
                                         function contRegisration5(selectedClass,subClientID){


                                                 var myform = $('#frm_add_participant');
                                                 var serialized = myform.serialize();
                                                 var datastring = serialized;

                                                 $.ajax({
                                                           type: &quot;POST&quot;,
                                                           timeout:6000000,
                                                           url: &quot;/PGCTechnologyServices/course-enrollments/registerforclasspart5?class_id=&quot;+selectedClass+'&amp;clientId='+subClientID+'&amp;RegisterAsFlag=1',
                                                           data: datastring,
                                                           beforeSend:function() {

                                                                //waitingDialog.hide();
                                                                //$('.ajax_instr_loader').hide();
                                                                //waitingDialog.show('Loading....');
                                                          },
                                                          complete: function() {

                                                                //waitingDialog.hide();
                                                                //$('.ajax_instr_loader').hide();
                                                                
                                                                $(&quot;#reg_status&quot;).val('done'); 
                                                                if($(&quot;#reguser&quot;).length>0){
                                                                    $(&quot;#reguser&quot;).remove();  
                                                                 }
                                                                
                                                            }, 
                                                           success: function(data) {
                                                                  data=jQuery.parseJSON(data);  
                                                                  if(data.error==&quot;true&quot;){

                                                                       if(data.hasOwnProperty('msg')){

                                                                               alert(data.msg);
                                                                         }
                                                                         else if(data.hasOwnProperty('message')){

                                                                              alert(data.message);

                                                                           }
                                                                  }
                                                                  else{

                                                                         if(data.hasOwnProperty('msg')){

                                                                               alert(data.msg);
                                                                         }
                                                                         else if(data.hasOwnProperty('message')){

                                                                              alert(data.message);

                                                                           }

                                                                  }


                                                          

                                                      // $.pjax.reload({container:'#new-model'});
                                                       
                                                       var magnificPopup = $.magnificPopup.instance; 
                                                        magnificPopup.close(); 
                                                        $(&quot;#participant&quot;).val('');
                                                        $(&quot;#participant_id&quot;).val('');
                                                        
                                                        $.pjax.reload({container:'#new-model',timeout:false}).done(function() { 
                                                        
                                                            $.pjax.reload({container:'#new-model-price',timeout:false}).done(function() { 

                                                                 $.magnificPopup.open({
                                                                     items: {
                                                                         src: '#add-participant' 
                                                                     },
                                                                     type: 'inline',
                                                                     closeOnBgClick:false,
                                                                     enableEscapeKey:false
                                                                 });
                                                                 
                                                                  $.pjax.reload({container:'#new-model-participant',timeout:false}).done(function() { 
                                                                        
                                                                         $.pjax.reload({container:'#new-model-certificate',timeout:false}).done(function() { 

                                                                                 $.pjax.reload({container:'#new-model-markall',timeout:false}).done(function() { 
                                                                                     
                                                                                      $.pjax.reload({container:'#new-model-waitlist',timeout:false}).done(function() { 
                                                                                     
                                                                                            $.pjax.reload({container:'#new-model-request',timeout:false}).done(function() { 
                                                                                                
                                                                                                    $.pjax.reload({container:'#eval_model',timeout:false}).done(function() { 

                                                                                                        if($(&quot;#reguser&quot;).length>0){
                                                                                                            $(&quot;#reguser&quot;).remove();  
                                                                                                         }
                                                                                                        $(&quot;#checked_participants_request&quot;).val('');
                                                                                                        $(&quot;#is_checked_all_request&quot;).val('');
                                                                                                        $(&quot;#unchecked_participants_request&quot;).val('');
                                                                                                        $(&quot;#declined_reason_main&quot;).val('');
                                                                                                    }) 
                                                                                              })
                                                                                            
                                                                                        })
                                                                                     
                                                                                 })
                                                                          })
                                                                    })
                                                                    
                                                                
                                                            })
        
    
                                                        })
                                                       
                                                        },
                                                           error: function (xhr, ajaxOptions, thrownError) {
                                                                //alert(xhr.status);
                                                                //alert(thrownError);
                                                                //waitingDialog.hide();
                                                                waitingDialog.hide();
                                                                $('.ajax_instr_loader').hide();
                                                                $(&quot;#hide_ajax_loader&quot;).val(&quot;false&quot;);

                                                            }

                                                   });

                                             }


                                             
                                         
                                             
                                             
                                          
                                    
                                
                                
                                
                                  
                                    
                                   function markThisAsComplete(currStatus,regid,classid,clientId,eve){
                                       eve.preventDefault();
                                       thisId=$(&quot;#reg_comp_&quot;+regid);
                                       
                                       if(currStatus==6){
                                           
                                           alert($(thisId).attr('data-name')+&quot; withdrew from the course. Approval no longer necessary&quot;)
                                           return false;
                                       }
                                       else{
                                           
                                           if(parseInt($(thisId).attr('data-class_status'))==8){
                                               
                                               alert(&quot;This class is cancelled. Approval no longer necessary&quot;);
                                               return false;
                                           }
                                           
                                       }
                                       
                                        
                                        datastring={};
                                        if(currStatus!=1){
                                            Msg = &quot;Are you sure you want to mark the selected participant complete?&quot;;
                                        }
                                        else{
                                            
                                            Msg = &quot;Selected participant already complete, Are you sure you want to mark the selected participant incomplete?&quot;;
                                        }
                                        var r = confirm(Msg);
                                        if (r === true) {
                                            
                                             $.ajax({
                                                        type: &quot;POST&quot;,
                                                        timeout:600000,
                                                        url: &quot;/PGCTechnologyServices/course-enrollments/marksinglecomplete?regid=&quot;+regid+&quot;&amp;classid=&quot;+classid+'&amp;clientid='+clientId,
                                                        data: datastring,
                                                        beforeSend:function() {

                                                             waitingDialog.hide();
                                                             $('.ajax_instr_loader').hide();
                                                             waitingDialog.show('Loading....');
                                                       },
                                                       complete: function() {

                                                             waitingDialog.hide();
                                                             $('.ajax_instr_loader').hide();
                                                         }, 
                                                        success: function(data) {
                                                               data=jQuery.parseJSON(data);  
                                                               if(data.error==&quot;true&quot;){

                                                                    if(data.hasOwnProperty('msg')){

                                                                            alert(data.msg);
                                                                      }
                                                                      else if(data.hasOwnProperty('message')){

                                                                           alert(data.message);

                                                                        }
                                                               }
                                                               else{

                                                                      if(data.hasOwnProperty('msg')){

                                                                            alert(data.msg);
                                                                      }
                                                                      else if(data.hasOwnProperty('message')){

                                                                           alert(data.message);

                                                                        }

                                                               }

                                                               $.pjax.reload({container:'#new-model',timeout:false}).done(function() {
                                                                   
                                                                    $.pjax.reload({container:'#new-model-price',timeout:false}).done(function() { 

                                                                         $.pjax.reload({container:'#new-model-participant',timeout:false}).done(function() { 
                                                                             
                                                                              $.pjax.reload({container:'#new-model-certificate',timeout:false}).done(function() { 
                                                                                  
                                                                                  
                                                                                   $.pjax.reload({container:'#new-model-markall',timeout:false}).done(function() { 

                                                                                          $.pjax.reload({container:'#new-model-waitlist',timeout:false}).done(function() { 
                                                                                     
                                                                                               $.pjax.reload({container:'#new-model-request',timeout:false}).done(function() { 
                                                                                                    
                                                                                                    $.pjax.reload({container:'#eval_model',timeout:false}).done(function() { 
                                                                                                            if($(&quot;#reguser&quot;).length>0){
                                                                                                               $(&quot;#reguser&quot;).remove();  
                                                                                                            }

                                                                                                            $(&quot;#checked_participants_request&quot;).val('');
                                                                                                           $(&quot;#is_checked_all_request&quot;).val('');
                                                                                                           $(&quot;#unchecked_participants_request&quot;).val('');
                                                                                                           $(&quot;#declined_reason_main&quot;).val('');
                                                                                                      })
                                                                                                })
                                                                                          
                                                                                        })
                                                                                        
                                                                                    })
                                                                       
                                                                                })
                                                                          })
                                                                    })
                                                                    
                                                                   
                                                                })

                                                        },
                                                        error: function (xhr, ajaxOptions, thrownError) {
                                                             //alert(xhr.status);
                                                             //alert(thrownError);
                                                             //waitingDialog.hide();
                                                             waitingDialog.hide();
                                                             $('.ajax_instr_loader').hide();
                                                             $(&quot;#hide_ajax_loader&quot;).val(&quot;false&quot;);

                                                         }

                                                   });
                                            
                                        }
                                       
                                   } 
                                   function markAllComplete(classid,ev,todo){
                                       
                                       ev.preventDefault();
                                       datastring={};
                                       if(todo=='allcomplete'){
                                                       
                                               
                                                Msg = &quot;Are you sure you want to mark all participants complete for this class?&quot;;
                                                
                                                
                                        }
                                        else{
                                            
                                                                                              
                                                Msg = &quot;Are you sure you want to mark all participants incomplete for this class?&quot;;
                                              
                                              
                                        }
                                        var r = confirm(Msg);
                                        if (r === true) {
                                            
                                                    $.ajax({
                                                            type: &quot;POST&quot;,
                                                            timeout:600000,
                                                            url: &quot;/PGCTechnologyServices/course-enrollments/markallcomplete?classid=&quot;+classid+'&amp;todo='+todo+'&amp;s='+$.trim($(&quot;#txt_searh&quot;).val()),
                                                            data: datastring,
                                                            beforeSend:function() {

                                                                 waitingDialog.hide();
                                                                 $('.ajax_instr_loader').hide();
                                                                 waitingDialog.show('Loading....');
                                                           },
                                                           complete: function() {

                                                                 waitingDialog.hide();
                                                                 $('.ajax_instr_loader').hide();
                                                             }, 
                                                            success: function(data) {
                                                                   data=jQuery.parseJSON(data);  
                                                                   if(data.error==&quot;true&quot;){

                                                                        if(data.hasOwnProperty('msg')){

                                                                                alert(data.msg);
                                                                          }
                                                                          else if(data.hasOwnProperty('message')){

                                                                               alert(data.message);

                                                                            }
                                                                   }
                                                                   else{

                                                                          if(data.hasOwnProperty('msg')){

                                                                                alert(data.msg);
                                                                          }
                                                                          else if(data.hasOwnProperty('message')){

                                                                               alert(data.message);

                                                                            }

                                                                   }

                                                                   $.pjax.reload({container:'#new-model',timeout:false}).done(function() {
                                                                   
                                                                        $.pjax.reload({container:'#new-model-price',timeout:false}).done(function() { 
                                                                            
                                                                             $.pjax.reload({container:'#new-model-participant',timeout:false}).done(function() { 
                                                                                 
                                                                                  $.pjax.reload({container:'#new-model-certificate',timeout:false}).done(function() { 
                                                                                      
                                                                                       $.pjax.reload({container:'#new-model-markall',timeout:false}).done(function() { 

                                                                                            $.pjax.reload({container:'#new-model-waitlist',timeout:false}).done(function() { 
                                                                                                
                                                                                                $.pjax.reload({container:'#new-model-request',timeout:false}).done(function() { 
                                                                                                        
                                                                                                    $.pjax.reload({container:'#eval_model',timeout:false}).done(function() {     
                                                                                                        if($(&quot;#reguser&quot;).length>0){
                                                                                                            $(&quot;#reguser&quot;).remove();  
                                                                                                         }
                                                                                                         $(&quot;#checked_participants_request&quot;).val('');
                                                                                                         $(&quot;#is_checked_all_request&quot;).val('');
                                                                                                         $(&quot;#unchecked_participants_request&quot;).val('');
                                                                                                         $(&quot;#declined_reason_main&quot;).val('');
                                                                                                     })  

                                                                                                })
                                                                                                

                                                                                           })
                                                                                        })
                                                                          
                                                                                  })
                                                                                    
                                                                             })
                                                                            
                                                                         })
                                                                    
                                                                    })

                                                            },
                                                            error: function (xhr, ajaxOptions, thrownError) {
                                                                 //alert(xhr.status);
                                                                 //alert(thrownError);
                                                                 //waitingDialog.hide();
                                                                 waitingDialog.hide();
                                                                 $('.ajax_instr_loader').hide();
                                                                 $(&quot;#hide_ajax_loader&quot;).val(&quot;false&quot;);

                                                             }

                                                       });


                                               } 
                                   
                                            }
                                            
                                            
                                   function markAllPaid(classid,ev,todo){
                                       
                                       ev.preventDefault();
                                       datastring={};
                                       if(todo=='allpaid'){
                                                       
                                               
                                                Msg = &quot;Are you sure you want to mark all participants paid for this class?&quot;;
                                                
                                                
                                        }
                                        else{
                                                                                              
                                                Msg = &quot;Are you sure you want to mark all participants unpaid for this class?&quot;;
                                              
                                              
                                        }
                                        var r = confirm(Msg);
                                        if (r === true) {
                                            
                                                    $.ajax({
                                                            type: &quot;POST&quot;,
                                                            timeout:600000,
                                                            url: &quot;/PGCTechnologyServices/course-enrollments/markallpaid?classid=&quot;+classid+'&amp;todo='+todo+'&amp;s='+$.trim($(&quot;#txt_searh&quot;).val()),
                                                            data: datastring,
                                                            beforeSend:function() {

                                                                 waitingDialog.hide();
                                                                 $('.ajax_instr_loader').hide();
                                                                 waitingDialog.show('Loading....');
                                                           },
                                                           complete: function() {

                                                                 waitingDialog.hide();
                                                                 $('.ajax_instr_loader').hide();
                                                             }, 
                                                            success: function(data) {
                                                                   data=jQuery.parseJSON(data);  
                                                                   if(data.error==&quot;true&quot;){

                                                                        if(data.hasOwnProperty('msg')){

                                                                                alert(data.msg);
                                                                          }
                                                                          else if(data.hasOwnProperty('message')){

                                                                               alert(data.message);

                                                                            }
                                                                   }
                                                                   else{

                                                                          if(data.hasOwnProperty('msg')){

                                                                                alert(data.msg);
                                                                          }
                                                                          else if(data.hasOwnProperty('message')){

                                                                               alert(data.message);

                                                                            }

                                                                   }

                                                                   $.pjax.reload({container:'#new-model',timeout:false}).done(function() {
                                                                   
                                                                        $.pjax.reload({container:'#new-model-price',timeout:false}).done(function() { 
                                                                            
                                                                             $.pjax.reload({container:'#new-model-participant',timeout:false}).done(function() { 
                                                                                 
                                                                                  $.pjax.reload({container:'#new-model-certificate',timeout:false}).done(function() { 
                                                                                      
                                                                                       $.pjax.reload({container:'#new-model-markall',timeout:false}).done(function() { 

                                                                                            $.pjax.reload({container:'#new-model-waitlist',timeout:false}).done(function() { 
                                                                                                
                                                                                                $.pjax.reload({container:'#new-model-request',timeout:false}).done(function() { 
                                                                                                        
                                                                                                    $.pjax.reload({container:'#eval_model',timeout:false}).done(function() {     
                                                                                                        if($(&quot;#reguser&quot;).length>0){
                                                                                                            $(&quot;#reguser&quot;).remove();  
                                                                                                         }
                                                                                                         $(&quot;#checked_participants_request&quot;).val('');
                                                                                                         $(&quot;#is_checked_all_request&quot;).val('');
                                                                                                         $(&quot;#unchecked_participants_request&quot;).val('');
                                                                                                         $(&quot;#declined_reason_main&quot;).val('');
                                                                                                     })  

                                                                                                })
                                                                                                

                                                                                           })
                                                                                        })
                                                                          
                                                                                  })
                                                                                    
                                                                             })
                                                                            
                                                                         })
                                                                    
                                                                    })

                                                            },
                                                            error: function (xhr, ajaxOptions, thrownError) {
                                                                 //alert(xhr.status);
                                                                 //alert(thrownError);
                                                                 //waitingDialog.hide();
                                                                 waitingDialog.hide();
                                                                 $('.ajax_instr_loader').hide();
                                                                 $(&quot;#hide_ajax_loader&quot;).val(&quot;false&quot;);

                                                             }

                                                       });


                                               } 
                                   
                                            }
                                  
                    
                    
                      
                  
                
               
            
        
    </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;js flexbox canvas canvastext webgl no-touch geolocation postmessage websqldatabase indexeddb hashchange history draganddrop websockets rgba hsla multiplebgs backgroundsize borderimage borderradius boxshadow textshadow opacity cssanimations csscolumns cssgradients cssreflections csstransforms csstransforms3d csstransitions fontface generatedcontent video audio localstorage sessionstorage webworkers no-applicationcache svg inlinesvg smil svgclippaths&quot;]/body[1]/div[@class=&quot;wrapper&quot;]/div[@class=&quot;main_content main_content_normal f-left&quot;]/div[@class=&quot;right-container&quot;]/div[@class=&quot;main-container pos-rel&quot;]/div[@class=&quot;center-cont&quot;]/div[@class=&quot;course-cont&quot;]/div[@class=&quot;course-inner-cont course-manager&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Logout'])[1]/following::div[10]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Edit Profile'])[1]/following::div[10]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div[3]/div</value>
   </webElementXpaths>
</WebElementEntity>
