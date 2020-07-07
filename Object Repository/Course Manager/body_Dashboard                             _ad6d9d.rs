<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Dashboard                             _ad6d9d</name>
   <tag></tag>
   <elementGuidId>bfd434bf-9126-477f-85a3-9afc1390a5e1</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>modal-open</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
        

        
        
            
                
                    
                        
                        

                                                               
                                        
                                

                                    
                                            
                                                                                                    
                                                  
                                            
                                        
                                    
                                    Dashboard

                                
                            
                                                            
                                      
                                

                                    
                                        
                                            
                                                                                                    
                                                                                                 
                                        
                                    
                                    Course Catalog
                                
                            
                                                            
                                      
                                

                                    
                                        
                                            
                                                                                                    
                                                                                                 
                                        
                                    
                                        
                                        Course History 2 
                                        
                                
                            


                                                            
                                   
                                                                 
                                    
                                        
                                            
                                                                                                    
                                                                                               
                                        
                                    
                                                                                                                        Pending Courses                         
                                            
                                            
                                
                            
                                                            
                                   
                                                                 
                                    
                                        
                                            
                                                                                                    
                                                                                               
                                        
                                    
                                                                                                                        Assigned Courses                         
                                            
                                            
                                
                            

                                                                                                                             
                                       
                                    

                                        
                                            
                                                
                                                                                                            
                                                                                                      
                                            
                                        
                                                                                    Pending Evaluations 
                                                                               
                                    
                                
                                  
                                                                        
                                               
                                            

                                                
                                                    
                                                        
                                                                  
                                                                
                                                            
                                                            
                                                                                                                      
                                                    
                                                
                                                Tools

                                            
                                        
                                                                    

                        
                    
                
            
            

                                        
                        
                        
                            
                                
                                    PGC County Schools
                                    


                                                                                    
                                                                                                        

                                                            
                                                                Online Users
                                                            

                                                        
                                                        

                                                            
                                                                Admin
                                                            

                                                        
                                                                                                            
                                                        
                                                            0 PLUs
                                                        
                                                    
                                                                                                    Hello, Alex Turner!
                                                


                                                    
                                                    
                                                
                                                
                                                    
                                                        
                                                            
                                                                Edit Profile
                                                                Logout
                                                            
                                                        
                                                    
                                                
                                            
                                            
                                    
                                
                            
                            
                                
                                    
                                    
                                    
                                
                            
                        
                                                
                                
                                    
                                            
                                    

                                                                                                                                                                
                                    
                                    Admin
Course Manager
Add Course


    
        New Course Request
    

waitingDialog.show('Loading....');
         
         
         
         
         
         
 
  
  

   var dateChange=false; 
   var finish_success=false;
   var firstTimeOnly=1;

  
    .spinner {
                width: 100px;
              }
              .spinner input {
                text-align: right;
              }
              .input-group-btn-vertical {
                position: relative;
                white-space: nowrap;
                width: 1%;
                vertical-align: middle;
                display: table-cell;
              }
              .input-group-btn-vertical > .btn {
                display: block;
                float: none;
                width: 100%;
                max-width: 100%;
                padding: 8px;
                margin-left: -1px;
                position: relative;
                border-radius: 0;
                height:17.5pt;
              }
              .input-group-btn-vertical > .btn:first-child {
                border-top-right-radius: 4px;
              }
              .input-group-btn-vertical > .btn:last-child {
                margin-top: -2px;
                border-bottom-right-radius: 4px;
              }
              .input-group-btn-vertical i{
                position: absolute;
                top: 4px;
                left: 4px;
              }
              fieldset.scheduler-border {
            border: solid 1px #DDD !important;
            padding: 0 10px 10px 10px;
            border-bottom: none;
          
        }

        legend.scheduler-border {
            width: auto !important;
            border: none;
            font-size: 14px;
            font-weight: bold;
        }
        .newlbl{color: #808080;font-family: Montserrat-Regular;font-size: 14px;margin: 0 0 10px;display: block;}
        #is_date_generated-error{color:red !important; font-size: 13px;font-family: unset !important}
        #is_trening_year_set-error{
            width:696px !important;
            max-width:696px !important;
        }
        #txtCustomHeader-error,#txtRegTerms-error{
            font-weight: normal;
            color: red !important;
            font-family: unset;
        }
        #numMaxClassSize,#numClassRoomHours,#numNonClassRoomHours,#NumRsvSeat{
            
            border-radius: 0px;
            text-align: left;
             width:320px;
            
        }
        
        #numMaxSizeExternal{
            width: 286px;
            text-align:left;
             border-radius: 0px;
        }
        #numMaxWaitList{
            width: 286px;
            text-align:left;
             border-radius: 0px;
        }
        #NumUnlimitReg{
            width: 104px;
            text-align:left;
             border-radius: 0px;
            
        }
        #NumRandDaysAfterReg{
            width: 104px;
            text-align:left;
             border-radius: 0px;
            
        }
        #numMinClassSize{
            width: 104px;
            text-align:left;
             border-radius: 0px;
            
        }
       


    
        
    
        
            
                
                    
                        
                            General Information
                            
                                General Information,
                                Description,
                                Instructor,
                                Training Center,
                                                                         Focus Areas,
                                  
                                        
                                    Custom Fields &amp;
                                    
                                Notifications
                            
                        
                        
                            Limits &amp; Schedule
                            
                                Limits,
                                Fees,
                                    
                                    
                                Schedule &amp;
                                Deadlines
                            
                        
                        
                            Restrictions
                            
                                Enrollment Restrictions,
                                                                     Target Audience,
                                    
                                                                    Rubrics
                                    
                                    
                                     &amp; Course Tracks
                                    
                            
                        
                                                    
                                Evaluation
                                
                                    Survey Questions
                                                                            &amp;Follow-up Questions
                                        
                                
                            
                                            
                
                
                
                     
                          
                             General Information 
                            
                            
                            
                                
                                    
                                        
                                            Course Title*
                                            
                                            
                                        
                                        
                                            Requested By
                                            
                                                    
                                                    
                                                


                                        
                                    
                                    
                                        
                                            Category*
                                            
                                                Choose..
                                                                                                District : ETC
                                                                                                District : learning plan
                                                                                                District : Technology Services
                                                                                                District: GLRS
                                                                                                District: Greene County
                                                                                                District: Learning Plan
                                                                                                District: NEGYSTC
                                                                                                District: PL
                                                                                                District: Prog Service
                                                                                                Local  : GAVS
                                                                                                NCS2
                                                                                                Purchasing
                                                                                                Ron philips
                                                                                                test 03
                                                                                                Testing
                                                                                            
                                            
                                        
                                        
                                            Course Type
                                            Choose...Blended Learning (Online/Face-to-Face)Online Learning (100% Virtual)Online Learning (Self-Paced/100% Virtual)Traditional Learning (Face-to-Face)
                                            
                                            
                                        
                                    
                                    

                                            

                                            
                                            Course Description Link
                                            
                                            
                                        
                                    
                                    
                                        
                                            Contact Telephone*
                                            
                                            
                                        
                                        
                                            Contact Email*
                                            
                                            
                                        
                                    
                                    
                                                                                
                                            Delivery Methods*
                                            
                                             Choose..
                                                                                                aptiris
                                                                                                Blended Classroom  (Online/Classroom)
                                                                                                Elluminate Sessions
                                                                                                GG
                                                                                                Mentoring/Face-to-Face Meeting
                                                                                                Online (100% Virtual)
                                                                                                Online (test)
                                                                                                Traditional Classroom (Face-to-Face)
                                                                                                Workshop
                                                                                            
                                            
                                        
                                                                                                                    
                                    
                                        
                                              
                                                                                            
                                                    Moodle Shortname
                                                    
                                                    
                                                  
                                                   
                                                                                            
                                                    Canvas Course ID
                                                    
                                                    
                                                  
                                                   
                                        
                                   
                                
                            
                        
                        
                            
                                                            
                                 $( &quot;#txtmoodle_shortnmae&quot; ).change(function() {
                                    
                                      $.ajax({
                                          type: &quot;POST&quot;,
                                          cache: false,
                                          timeout:60000,
                                          url: &quot;/PGCTechnologyServices/course-manager/ismoodleclassexist?moodleShortname=&quot;+$( &quot;#txtmoodle_shortnmae&quot; ).val()+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime(),
                                          beforeSend:function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                             waitingDialog.show('Loading....');
                                          },    
                                          success: function(data) {
                                              data=jQuery.parseJSON(data);
                                              if(data.result==&quot;0&quot;){
                                                  
                                                    alert(&quot;Ooops, it would appear we cannot find this moodle class. Please check for spelling errors.&quot;);   
                                                    $(&quot;#txtmoodle_shortnmae&quot;).val('');
                                              }
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                          },
                                           error: function (xhr, ajaxOptions, thrownError) {
                                              //alert(xhr.status);
                                              //alert(thrownError);
                                              //waitingDialog.hide();
                                              waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                         },
                                         complete: function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                         }

                                      });
                                 
                                });
                                
                                
                                       
                         
                                                           
                                 $( &quot;#txtcanvas_courseid&quot; ).change(function() {
                                    
                                      $.ajax({
                                          type: &quot;POST&quot;,
                                          cache: false,
                                          timeout:60000,
                                          url: &quot;/PGCTechnologyServices/course-manager/iscanvasclassexist?canvas_course_id=&quot;+$( &quot;#txtcanvas_courseid&quot; ).val()+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime(),
                                          beforeSend:function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                             waitingDialog.show('Loading....');
                                          },    
                                          success: function(data) {
                                              data=jQuery.parseJSON(data);
                                              if(data.result==&quot;0&quot;){
                                                  
                                                    alert(&quot;Ooops, it would appear we cannot find this canvas class. Please check canvas id.&quot;);   
                                                    $(&quot;#txtcanvas_courseid&quot;).val('');
                                              }
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                          },
                                           error: function (xhr, ajaxOptions, thrownError) {
                                              //alert(xhr.status);
                                              //alert(thrownError);
                                              //waitingDialog.hide();
                                              waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                         },
                                         complete: function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                         }

                                      });
                                 
                                });
                                
                                
                                
                             $.validator.setDefaults({ ignore: '' });
                              $('#frm_generalinfo').validate({

                                    rules: {
                                            txtTitle: {
                                                    required: true,
                                                    maxlength:100

                                            },
                                            cboCategory: {
                                                    required: true

                                            },
                                            course_category: {
                                                    required: true

                                            },
                                            
                                            txtPhone1:{
                                                
                                                 
                                                  required:true             
                                                                                                      
                                            },
                                            txtContactEmail1: {
                                                    email: true
                                                     
                                                        ,
                                                    required:true
                                                                                                },
                                            txtCourseDescriptionURL: {
                                                    
                                                    url: true
                                            }
                                                                                       
                                            ,
                                             cboDeliveryMethod: {
                                                   required: true

                                           }
                                            
                                                
                                    },
                                     errorPlacement: function(error, element) {
                                            error.appendTo( element.next());
                                      },
                                     messages: {
                                         is_valid_comp_num:{
                                             
                                             required: &quot;Please enter valid component numbers&quot;,
                                         }
                                     }

                                });   
                        
                            
                    
                    
                         General InformationDescription
                        
                           
                            
                            
                            
                                
                                    Course Image
                                    
                                        
                                            
                                            
                                            function readURL_course_img(input) {
                                                    if (input.files &amp;&amp; input.files[0]) {

                                                         var fileToLoad = input.files[0];
                                                         var filetype = fileToLoad.type.toLowerCase();

                                                         if (filetype == 'image/jpeg' || filetype == 'image/png') {

                                                            var reader = new FileReader();
                                                            reader.onload = function (e) {
                                                                    $('#output').attr('src', e.target.result);
                                                                    $('#course_img').val(e.target.result);
                                                            };
                                                            reader.readAsDataURL(input.files[0]);

                                                        }
                                                        else{

                                                            alert(&quot;Please only upload png or jpeg image only&quot;);
                                                            return false;

                                                        }
                                                    }
                                            }
                                        
                                            
                                            
                                        
                                         
                                            
                                                
                                                    BROWSE
                                                    
                                                

                                            
                                        
                                    
                                    
                                
                                 
                                
                                    
                                        Course Description (Please use CTRL+V to paste)*
                                        
                                            
                                              Times New Roman 12pt  Formats   p
                                              
                                            
                                               function insert_contents(inst){
                                                 inst.setContent($(&quot;#txtCourseDetail&quot;).val());  
                                                    
                                                }
                                                tinymce.init({
                                                    selector: '#txtCourseDetail',
                                                    setup: function (editor) {
                                                        editor.on('change', function () {
                                                            editor.save();
                                                        });
                                                        
                                                        
                                                    },
                                                    init_instance_callback: &quot;insert_contents&quot;,
                                                    height: 200,
                                                    width:670,
                                                    menubar: false,
                                                    plugins: [
                                                      'advlist autolink lists charmap print preview anchor',
                                                      'searchreplace visualblocks code fullscreen',
                                                      'insertdatetime  table contextmenu paste code',
                                                      'textcolor','link'
                                                    ],
                                                    toolbar: 'forecolor backcolor | fontselect fontsizeselect | undo redo | insert | styleselect | bold italic  | alignleft aligncenter alignright alignjustify | bullist numlist outdent indent link',
                                                    fontsize_formats: &quot;8pt 10pt 12pt 14pt 18pt 24pt 36pt&quot;,
                                                    textcolor_cols: &quot;5&quot;

                                                  
                                                  });
                                                  
                                                  

                                             
                                            
                                            
                                            
                                        
                                    
                                

                            
                        
                         
                             $.validator.setDefaults({ ignore: '' });
                             
                               $.validator.addMethod(&quot;checkContent&quot;, function(value, element) {
                                    
                                        if(tinyMCE.get('txtCourseDetail')!=null){
                                            var editorContent = tinyMCE.get('txtCourseDetail').getContent();
                                        }
                                        else{
                                            
                                              var editorContent =$(&quot;#txtCourseDetail&quot;).val();
                                        }
                                        
                                        if (editorContent == '')
                                        {
                                            return false;
                                        }
                                        else
                                        {
                                            return true;
                                        }
                                        
                               }, &quot;Please enter the course description&quot;);

                              $('#frm_course_description').validate({

                                        rules: {
                                                is_valid_description: {
                                                        checkContent: true

                                                }

                                        },
                                         errorPlacement: function(error, element) {
                                                error.appendTo( element.next());
                                          }

                                    });   
                            
                              
                    
                    
                         General InformationInstructor
                        
                           
                             
                             
                            
                                
                                    
                                        
                                            
                                                  Add Instructor
                                            
                                        
                                    
                                
                                
                                        
                                            
                                                Instructor
                                                Email
                                                Telephone
                                                Action
                                            
                                        
                                        
                                              No Instructor                                        
                                
                                
                                
                            
                        
                             
                                $.validator.setDefaults({ ignore: '' });

                                  $.validator.addMethod(&quot;checkInstructors&quot;, function(value, element) {

                                           
                                           if ($.trim($(&quot;#instructor_tbody&quot;).text()) == 'No Instructor')
                                           {
                                               return false;
                                           }
                                           else
                                           {
                                               return true;
                                           }

                                  }, &quot;Please add at least one Instructor&quot;);

                                 $('#frm_instruction').validate({

                                           rules: {
                                                   is_instructor_added: {
                                                           checkInstructors: true

                                                   }

                                           },
                                            errorPlacement: function(error, element) {
                                                   error.appendTo( element.next());
                                             }

                                       });   
                            
                           
                    
                    
                         General InformationTraining Center
                        
                         
                            
                            
                            
                                
                                    
                                        Training Center *
                                        
                                            Choose
                                                                                             Arbor Connection Conference Center
                                                                                             Brasstown Valley Resort
                                                                                             C.W. Davis Middle School
                                                                                             Chandigarh
                                                                                             Chestatee High School
                                                                                             Chestnut Mt Elementary School 
                                                                                             Civic Center
                                                                                             Cousins Middle School
                                                                                             DaVinci Academy at South Hall Middle
                                                                                             Dawson County BOE
                                                                                             Dawson County Middle School
                                                                                             Forsyth County BOE
                                                                                             Franklin County BOE
                                                                                             Gainesville Civic Center
                                                                                             Gainesville Exploration Academy
                                                                                             GAVS Specific
                                                                                             Georgia World Congress Center
                                                                                             Georgia World Congress Center 2
                                                                                             GG
                                                                                             GG 01
                                                                                             Gordon Street Center
                                                                                             Habersham Central High School
                                                                                             Hartwell Elementary School
                                                                                             Indian Creek Middle School
                                                                                             Jackson County BOE
                                                                                             Jackson EMC
                                                                                             Johnson High School
                                                                                             Kennesaw State University
                                                                                             Lumpkin County BOE
                                                                                             Lumpkin County Middle School
                                                                                             My New Test LOC
                                                                                             Newton County School Board Office
                                                                                             Oakwood Elementary School
                                                                                             Online Courses
                                                                                             Other
                                                                                             Park PLace
                                                                                             Peachtree CEnter
                                                                                             Rabun County MS/HS
                                                                                             Sardis Elementary School
                                                                                             Sardis Enrichment School &amp; Martin Technology Academy
                                                                                             Sawnee Elementary School
                                                                                             Shiloh Point Elementary School
                                                                                             Spout Springs Elementary School
                                                                                             Test 01
                                                                                             University of West Georgia Humanities Building
                                                                                             Veterans Memorial Middle School
                                                                                             Vvvv 11
                                                                                             West Hall High School
                                                                                             White County Middle School
                                                                                             Woodland hills
                                                                                             World Language Academy
                                                                                             XXXXX Room
                                                                                     
                                        
                                    
                                    
                                        Room*
                                        
                                            Choose
                                        
                                        
                                    
                                
                                
                                    
                                     Add your Training Center
                                    
                                    
                                        
                                            Training Center Name*
                                            
                                            
                                        
                                        
                                            Room Name*
                                            
                                            
                                        
                                    
                                    
                                        
                                            Address 1
                                            
                                            
                                        
                                        
                                            Address 2
                                            
                                            
                                        
                                    
                                    
                                        
                                            City
                                            
                                            
                                        
                                        
                                            State
                                            
                                            
                                        
                                    
                                    
                                        
                                            Postal Code
                                            
                                            
                                        
                                        
                                            Direction URL
                                            
                                            
                                        
                                    
                                   
                                
                            

                        
                        
                            $.validator.setDefaults({ ignore: '' });
                            $('#frm_locations').validate({

                                  rules: {
                                          cboCourseLocation: {
                                                  required: true

                                          },
                                          cboCourseLocRoom: {
                                                  required: true

                                          },
                                          txtDirectionsURL: {
                                                  url: true

                                          }

                                  },
                                   errorPlacement: function(error, element) {
                                          error.appendTo( element.next());
                                    }

                              });   
                      
                            
                    
                                        
                        
                              
                                
                                 General InformationFocus Areas
                                
                                    
                                        
                                            
                                                
                                                    
                                                        
                                                              Add Focus Area
                                                        
                                                    
                                                
                                            
                                            
                                                
                                                    
                                                        Focus Area
                                                            
                                                        Action
                                                    
                                                
                                                

                                                     No Focus Area
                                                
                                            
                                            
                                            
                                        
                                    
                              
                            
                            $.validator.setDefaults({ ignore: '' });

                                                            
                                 $.validator.addMethod(&quot;checkfocusArea&quot;, function(value, element) {

                                           
                                           if ($.trim($(&quot;#focus_area_body&quot;).text()) == 'No Focus Area')
                                           {
                                               return false;
                                           }
                                           else
                                           {
                                               return true;
                                           }

                                  }, &quot;Please add at least one focus area&quot;);
                               
                                
                                

                             $('#frm_focusarea').validate({

                                       rules: {
                                               is_focus_area_done: {
                                                       checkfocusArea: true

                                               }

                                       },
                                        errorPlacement: function(error, element) {
                                               error.appendTo( element.next());
                                         }

                                   });   
                        
                        
                        
                   
                                                                
                         
                            
                             General InformationCustom Fields
                            
                            
                                function setValue(elem){
                                   
                                   if($(elem).is(&quot;:checked&quot;)){
                                       
                                       $(elem).val('1');
                                   }
                                   else{
                                       
                                       $(elem).val('0');
                                       
                                   }
                                   
                                    
                                }
                            
                            
                                
                                    
                                        
                                            
                                              
                                              
                                                      Textbook Provided*
                                                      
                                                        Choose..NANoYes                                                     
                                              
                                              
                                              
                                                      SD#
                                                      
                                                                                                             
                                              
                                              
                                              
                                                      Migrated Course?
                                                      
                                                                                                             
                                              
                                              
                                              
                                                      Learning Outcomes
                                                      
                                                                                                             
                                              
                                                                                    
                                    
                                
                            
                            
                          
                        
                        
                            
                            $( &quot;.cust_fld&quot; ).change(function() {
                                
                                $(&quot;#hdn_is_changed&quot;).val('changed');
                            });
                            $.validator.setDefaults({ ignore: '' });

                                
                            $('#frm_customfields').validate({

                                        errorPlacement: function(error, element) {
                                               error.appendTo( element.next());
                                         }

                                   }); 
                        
                    
                                        
                         
                             General InformationCourse Notifications
                            
                            
                            
                                
                                    
                                        
                                            
                                                Custom message footer for each notification.
                                                
                                                    function showHideAddCustomMessage(){
                                                       
                                                       if($(&quot;#isMsgHeaderInd&quot;).val()=='0'){
                                                           
                                                          $(&quot;#AddCustomMessage&quot;).show(); 
                                                          $(&quot;#isMsgHeaderInd&quot;).val('1');
                                                          $(&quot;#chkmark2&quot;).removeClass().addClass('i-check fa fa-check');
                                                          
                                                       }
                                                       else{
                                                           
                                                           $(&quot;#AddCustomMessage&quot;).hide();
                                                           $(&quot;#isMsgHeaderInd&quot;).val('0');
                                                           $(&quot;#chkmark2&quot;).removeClass().addClass('i-check fa fa-times');
                                                       }
                                                        
                                                    }
                                                   
                                                    
                                                
                                            
                                            
                                                
                                                      Times New Roman 12pt  Formats   pThis form must be signed and dated by the participant's supervisor. This SIGNED form must be mailed, faxed or brought to our office by the first course date.  If you are responsible for payment, participant payments will be collected at the first course meeting. 

Participant's Name ________________________________________________________________

Supervisor's Name ________________________________________________________________

Supervisor's Signature ____________________________________________________________

Date __________________________
                                                    
                                                          
                                                        tinymce.init({
                                                            selector: '#txtCustomHeader',
                                                            setup: function (editor) {
                                                                editor.on('change', function () {
                                                                    editor.save();
                                                                    $(&quot;#txtCustomHeader&quot;).html(tinyMCE.get('txtCustomHeader').getContent())

                                                                });
                                                            },
                                                            height: 200,
                                                            width:670,
                                                            menubar: false,
                                                            plugins: [
                                                              'advlist autolink lists charmap print preview anchor',
                                                              'searchreplace visualblocks code fullscreen',
                                                              'insertdatetime  table contextmenu paste code',
                                                              'textcolor','link'
                                                            ],
                                                            toolbar: 'forecolor backcolor | fontselect fontsizeselect | undo redo | insert | styleselect | bold italic  | alignleft aligncenter alignright alignjustify | bullist numlist outdent indent link',
                                                            fontsize_formats: &quot;8pt 10pt 12pt 14pt 18pt 24pt 36pt&quot;,
                                                            textcolor_cols: &quot;5&quot;    
                                                          });
                                                          
                                                          // showHideAddCustomMessage();
                                                       
                                                
                                            
                                        
                                    
                                
                                
                                
                                    
                                        
                                            
                                                Override Registration Terms
                                                
                                                    function IsOverrideRegTermsfunc(vals){
                                                        
                                                         if($(&quot;#IsOverrideRegTerms&quot;).val()=='0'){
                                                           
                                                          $(&quot;#OverrideRegistrationTerms&quot;).show(); 
                                                          $(&quot;#IsOverrideRegTerms&quot;).val('1');
                                                          $(&quot;#chkmark1&quot;).removeClass().addClass('i-check fa fa-check');
                                                           
                                                        }
                                                        else{

                                                            $(&quot;#OverrideRegistrationTerms&quot;).hide();
                                                            $(&quot;#IsOverrideRegTerms&quot;).val('0');
                                                            $(&quot;#chkmark1&quot;).removeClass().addClass('i-check fa fa-times');
                                                        }
                                                        
                                                    }
                                                    
                                                    
                                                
                                            
                                            
                                                
                                                    
                                                        Times New Roman 12pt  Formats   pnull
                                                      
                                                              
                                                            tinymce.init({
                                                                selector: '#txtRegTerms',
                                                                setup: function (editor) {
                                                                    editor.on('change', function () {
                                                                        editor.save();
                                                                        $(&quot;#IsOverrideRegTerms&quot;).html(tinyMCE.get('IsOverrideRegTerms').getContent())
                                                                    });
                                                                },
                                                                height: 200,
                                                                width:670,
                                                                menubar: false,
                                                                plugins: [
                                                                  'advlist autolink lists charmap print preview anchor',
                                                                  'searchreplace visualblocks code fullscreen',
                                                                  'insertdatetime  table contextmenu paste code',
                                                                  'textcolor','link'
                                                                ],
                                                                toolbar: 'forecolor backcolor | fontselect fontsizeselect | undo redo | insert | styleselect | bold italic  | alignleft aligncenter alignright alignjustify | bullist numlist outdent indent link',
                                                                fontsize_formats: &quot;8pt 10pt 12pt 14pt 18pt 24pt 36pt&quot;,
                                                                textcolor_cols: &quot;5&quot;    
                                                              });
                                                              
                                                              
                                                           
                                                    
                                                
                                            
                                                                                        
                                                
                                                if($(&quot;#IsOverrideRegTerms&quot;).val()=='1'){
                                                           
                                                    $(&quot;#OverrideRegistrationTerms&quot;).show(); 
                                                    $(&quot;#chkmark1&quot;).removeClass().addClass('i-check fa fa-check');

                                                  }
                                                  else{
                                                      
                                                        $(&quot;#OverrideRegistrationTerms&quot;).hide(); 
                                                       $(&quot;#chkmark1&quot;).removeClass().addClass('i-check fa fa-times');
                                                  }
                                                  
                                                if($(&quot;#isMsgHeaderInd&quot;).val()=='1'){

                                                    $(&quot;#AddCustomMessage&quot;).show(); 
                                                    $(&quot;#chkmark2&quot;).removeClass().addClass('i-check fa fa-check');

                                                 }
                                                 else{

                                                     $(&quot;#AddCustomMessage&quot;).hide();
                                                     $(&quot;#chkmark2&quot;).removeClass().addClass('i-check fa fa-times');
                                                   }
                                                   
                                            
                                                                                    
                                    
                                
                            
                          
                    
                    
                     
                         
                         
                         Limits &amp; ScheduleRegistration
                        
                        
                            
                                
                                    Registration Type : 
                                    Auto ApproveRegistration requires approvalMinimum enrollment Required
                                    
                                
                                
                                
                                    Registration requires approval
                                    
                                        
                                            
                                                 
                                                
                                            
                                            
                                                  Allow  
                                            
                                            
                                                  
                                                                                                                                                                                     
                                                                    
                                                                    
                                                                        Unlimited
                                                                    
                                                                   
      
                                                                
                                                                  
                                                                  
                                                                
                                                                  
                                                   

                                                        $(function() {
                                                          $('.NumUnlimitReg_element .dropdown-menu a').click(function() {

                                                              $(this).closest('.dropdown').find('#NumUnlimitReg').val($(this).attr('data-value'));
                                                              $('#NumUnlimitReg').trigger('blur');

                                                          });

                                                        }); 
                                                       $( &quot;#btn_up&quot; ).click(function() {

                                                              if(isNaN($(&quot;#NumUnlimitReg&quot;).val())){

                                                                $(&quot;#NumUnlimitReg&quot;).val('0');
                                                              }
                                                              $(&quot;#NumUnlimitReg&quot;).val(parseInt($(&quot;#NumUnlimitReg&quot;).val())+1);
                                                              $(&quot;#ChkAllowUnlimitReg&quot;).prop('checked', true);
                                                              $( &quot;#ChkAllowUnlimitReg&quot; ).val(1);
                                                              $(&quot;#2stcheckmark&quot;).show();

                                                          
                                                       });

                                                       $( &quot;#btn_down&quot; ).click(function() {

                                                      
                                                             if(isNaN($(&quot;#NumUnlimitReg&quot;).val())){

                                                                $(&quot;#NumUnlimitReg&quot;).val('0');
                                                              }
                                                              if(parseInt($(&quot;#NumUnlimitReg&quot;).val())-1 >=0){
                                                                  
                                                                 $(&quot;#NumUnlimitReg&quot;).val(parseInt($(&quot;#NumUnlimitReg&quot;).val())-1);
                                                              }
                                                              $(&quot;#ChkAllowUnlimitReg&quot;).prop('checked', true);
                                                              $( &quot;#ChkAllowUnlimitReg&quot; ).val(1);
                                                              $(&quot;#2stcheckmark&quot;).show();

                                                          
                                                       });

                                                   

                                               
                                                
                                            
                                            
                                            registrations beyond max defined seats
                                            
                                        
                                        
                                    
                                    
                                        
                                            
                                                 
                                                
                                            
                                            
                                                  Automatically approve max allowed participants randomly  
                                            
                                            
                                               
                                                  
                                                      
                                                    
                                                    
                                                      
                                                      
                                                    
                                                   

                                                       $('#ChkRegRandom').click(function() {
                                                          
                                                          if($('#ChkRegRandom').is(&quot;:checked&quot;)){
                                                              
                                                              $('#ChkRegRandom').val(1);
                                                          }
                                                          else{
                                                              $('#ChkRegRandom').val(0);
                                                          }
                                                       });
                                                       
                                                       $( &quot;#btn_up_1&quot; ).click(function() {

                                                             if(isNaN($(&quot;#NumRandDaysAfterReg&quot;).val())){

                                                                $(&quot;#NumRandDaysAfterReg&quot;).val('0');
                                                              }

                                                              $(&quot;#NumRandDaysAfterReg&quot;).val(parseInt($(&quot;#NumRandDaysAfterReg&quot;).val())+1);
                                                              $('#ChkRegRandom').prop('checked', true);
                                                              $('#ChkRegRandom').val(1);
                                                              
                                                       });

                                                       $( &quot;#btn_down_1&quot; ).click(function() {

                                                            if(isNaN($(&quot;#NumRandDaysAfterReg&quot;).val())){

                                                               $(&quot;#NumRandDaysAfterReg&quot;).val('0');
                                                             }
                                                              if($(&quot;#NumRandDaysAfterReg&quot;).val()-1>0){
                                                                   $(&quot;#NumRandDaysAfterReg&quot;).val(parseInt($(&quot;#NumRandDaysAfterReg&quot;).val())-1);
                                                                   $('#ChkRegRandom').prop('checked', true);
                                                                   $('#ChkRegRandom').val(1);
                                                                }
                                                       });

                                                   

                                               
                                                
                                            
                                            
                                             days after registration deadline
                                            
                                        
                                    
                                    
                                        
                                        
                                             
                                                 
                                                
                                            
                                            
                                                  Require Work location approval  
                                            
                                        
                                        
                                             
                                                 
                                                
                                            
                                            
                                                Require District approval  
                                               
                                            
                                        
                                        
                                             
                                                 
                                                
                                            
                                            
                                                Require Global level approval  
                                                
                                                    
                                                
                                                 
                                                 

                                                       $('#RequireWorkLocApproval').click(function() {
                                                          
                                                          if($('#RequireWorkLocApproval').is(&quot;:checked&quot;)){
                                                              
                                                              $('#RequireWorkLocApproval').val(1);
                                                          }
                                                          else{
                                                              $('#RequireWorkLocApproval').val(0);
                                                          }
                                                       });
                                                       
                                                       $('#RequireDistrictApproval').click(function() {
                                                          
                                                          if($('#RequireDistrictApproval').is(&quot;:checked&quot;)){
                                                              
                                                              $('#RequireDistrictApproval').val(1);
                                                          }
                                                          else{
                                                              $('#RequireDistrictApproval').val(0);
                                                          }
                                                       });
                                                       $('#RequireGlobalApproval').click(function() {
                                                          
                                                          if($('#RequireGlobalApproval').is(&quot;:checked&quot;)){
                                                              
                                                              $('#RequireGlobalApproval').val(1);
                                                          }
                                                          else{
                                                              $('#RequireGlobalApproval').val(0);
                                                          }
                                                       });
                                                       
                                            
                                        
                                        
                                             
                                         Send Enrollment requests to* 
                                            
                                            
                                                
                                                    
                                                
                                                
                                            
                                        
                                         
                                             
                                                 Enrollment Request Notification*
                                            
                                            
                                                
                                                    
                                                
                                                
                                            
                                        
                                       
                                    
                                    
                                
                                
                                    Minimum enrollment Required
                                    
                                        
                                              
                                          Minimum Class Size*  
                                             
                                            
                                                 
                                                     
                                                     
                                                     
                                                        
                                                          
                                                          
                                                        
                                                    
                                                    

                                                        $( &quot;#btn_up_2&quot; ).click(function() {
                                                            
                                                             if(isNaN($(&quot;#numMinClassSize&quot;).val())){

                                                                $(&quot;#numMinClassSize&quot;).val('0');
                                                              }
                                                              
                                                              $(&quot;#numMinClassSize&quot;).val(parseInt($(&quot;#numMinClassSize&quot;).val())+1);
                                                               
                                                        });

                                                        $( &quot;#btn_down_2&quot; ).click(function() {

                                                              if(isNaN($(&quot;#numMinClassSize&quot;).val())){

                                                                    $(&quot;#numMinClassSize&quot;).val('0');
                                                              }
                                                              
                                                             if(parseInt($(&quot;#numMinClassSize&quot;).val())-1 >=0){
                                                                 
                                                                 $(&quot;#numMinClassSize&quot;).val(parseInt($(&quot;#numMinClassSize&quot;).val())-1);
                                                             }
                                                              
                                                        });

                                                    

                                                
                                                
                                            
                                        
                                      
                                    
                                    
                                        
                                            
                                                
                                                
                                                    
                                                      
                                                         
                                                             Auto Send Confirmation when minimum enrollment is met 
                                                        
                                                        
                                                            Manually send confirmation messages from enrollment tool
                                                        
                                                         
                                                      
                                                  
                                              
                                                
                                            
                                            
                                               Confirmation Message 
                                               
                                            
                                        
                                    
                                    
                                
                                
                                    
                                    
                                        
                                            Maximum Class Size
                                            
                                                
                                                
                                                
                                                
                                                  
                                                  
                                                
                                                

                                                    $( &quot;#btn_up_numMaxClassSize_opt&quot; ).click(function() {
                                                        
                                                        
                                                             if(isNaN($(&quot;#numMaxClassSize&quot;).val())){

                                                                $(&quot;#numMaxClassSize&quot;).val('0');
                                                              }


                                                           $(&quot;#numMaxClassSize&quot;).val(parseInt($(&quot;#numMaxClassSize&quot;).val())+1);
                                                      

                                                    });

                                                    $( &quot;#btn_down_numMaxClassSize_opt&quot; ).click(function() {

                                                         if(isNaN($(&quot;#numMaxClassSize&quot;).val())){

                                                            $(&quot;#numMaxClassSize&quot;).val('0');
                                                          }
                                                         if(parseInt($(&quot;#numMaxClassSize&quot;).val())-1 >0){
                                                             
                                                             $(&quot;#numMaxClassSize&quot;).val(parseInt($(&quot;#numMaxClassSize&quot;).val())-1);
                                                      
                                                         }  

                                                    });
                                                        
                                                    $(&quot;input:radio[name='rbnAuto']&quot;).click(function(){
                                                        
                                                        if(this.id==&quot;auto&quot;){
                                                            
                                                            $(this).val(0);
                                                        }
                                                        else{
                                                             $(this).val(1);
                                                        }
                                                    })
                                                    
                                                    
                                                

                                            
                                            
                                        
                                        
                                            Class Room Hours
                                             
                                                 
                                                 
                                                
                                                
                                                  
                                                  
                                                
                                                

                                                    $( &quot;#btn_up_numClassRoomHours&quot; ).click(function() {

                                                            if(isNaN($(&quot;#numClassRoomHours&quot;).val())){

                                                                $(&quot;#numClassRoomHours&quot;).val('0');
                                                              }
 
                                                           $(&quot;#numClassRoomHours&quot;).val(parseInt($(&quot;#numClassRoomHours&quot;).val())+1);
                                                          
                                                           $('#numClassRoomHours').trigger('change');

                                                    });

                                                    $( &quot;#btn_down_numClassRoomHours&quot; ).click(function() {

                                                        if(isNaN($(&quot;#numClassRoomHours&quot;).val())){

                                                            $(&quot;#numClassRoomHours&quot;).val('0');
                                                          }
                                                         if(parseInt($(&quot;#numClassRoomHours&quot;).val())-1 >=0){
                                                             
                                                             $(&quot;#numClassRoomHours&quot;).val(parseInt($(&quot;#numClassRoomHours&quot;).val())-1);
                                                        
                                                             $('#numClassRoomHours').trigger('change');
                                                         }  

                                                    });

                                                

                                            
                                            
                                        
                                    
                                    
                                        
                                                                                                                                    Maximum External Participant
                                            
                                                
                                                
                                                    
                                                    
                                                     Unlimited
                                                   
                                                   
                                                    
                                                         
                                                         
                                                     
                                                
                                                  

                                                         $(function() {
                                                            $('.numMaxSizeExternal_element .dropdown-menu a').click(function() {

                                                                $(this).closest('.dropdown').find('#numMaxSizeExternal').val($(this).attr('data-value'));
                                                                $('#numMaxSizeExternal').trigger('blur');
                                                              
                                                            });

                                                          });
                                                       $( &quot;#btn_up_numMaxSizeExternal&quot; ).click(function() {


                                                            if(isNaN($(&quot;#numMaxSizeExternal&quot;).val())){

                                                                $(&quot;#numMaxSizeExternal&quot;).val('0');
                                                            }
                                                            $(&quot;#numMaxSizeExternal&quot;).val(parseInt($(&quot;#numMaxSizeExternal&quot;).val())+1);

                                                       });

                                                       $( &quot;#btn_down_numMaxSizeExternal&quot; ).click(function() {

                                                           if(isNaN($(&quot;#numMaxSizeExternal&quot;).val())){

                                                                $(&quot;#numMaxSizeExternal&quot;).val('0');
                                                             }
                                                             if(parseInt($(&quot;#numMaxSizeExternal&quot;).val())-1 >=0){

                                                                 $(&quot;#numMaxSizeExternal&quot;).val(parseInt($(&quot;#numMaxSizeExternal&quot;).val())-1);
                                                             }   
                                                             
                                                     
                                                       });

                                                   

                                               
                                               
                                        
                                        
                                            Non-Classroom Hours
                                            
                                                
                                               
                                                
                                                  
                                                  
                                                
                                                

                                                    $( &quot;#btn_up_numNonClassRoomHours&quot; ).click(function() {

                                                            if(isNaN($(&quot;#numNonClassRoomHours&quot;).val())){

                                                                $(&quot;#numNonClassRoomHours&quot;).val('0');
                                                              }
 
                                                           $(&quot;#numNonClassRoomHours&quot;).val(parseInt($(&quot;#numNonClassRoomHours&quot;).val())+1);
                                                           //$(&quot;#opt_numNonClassRoomHours&quot;).text(parseInt($(&quot;#opt_numNonClassRoomHours&quot;).val()));

                                                           //$('#numNonClassRoomHours option:eq(0)').prop('selected', true);
                                                           $(&quot;#numNonClassRoomHours&quot;).trigger('change');

                                                    });

                                                    $( &quot;#btn_down_numNonClassRoomHours&quot; ).click(function() {

                                                        if(isNaN($(&quot;#numNonClassRoomHours&quot;).val())){

                                                            $(&quot;#numNonClassRoomHours&quot;).val('0');
                                                          }
                                                         if(parseInt($(&quot;#numNonClassRoomHours&quot;).val())-1 >=0){
                                                             
                                                             $(&quot;#numNonClassRoomHours&quot;).val(parseInt($(&quot;#numNonClassRoomHours&quot;).val())-1);
                                                             //$(&quot;#opt_numNonClassRoomHours&quot;).text(parseInt($(&quot;#opt_numNonClassRoomHours&quot;).val()));

                                                             //$('#numNonClassRoomHours option:eq(0)').prop('selected', true);
                                                             $(&quot;#numNonClassRoomHours&quot;).trigger('change');
                                                         }  

                                                    });

                                                

                                            
                                            
                                        
                                    
                                    
                                        
                                                                                                                                    
                                            Maximum Wait List
                                              
                                           
                                                   
                                                        

                                                        
                                                           Unlimited
                                                           No Wait List
                                                         
                                                         
                                                         
                                                           
                                                           
                                                         
                                                        
                                                   

                                                       $(function() {
                                                            $('.numMaxWaitList_element .dropdown-menu a').click(function() {

                                                                $(this).closest('.dropdown').find('#numMaxWaitList').val($(this).attr('data-value'));
                                                                $('#numMaxWaitList').trigger('blur');
                                                              
                                                            });

                                                          });
                                                       $( &quot;#btn_up_numMaxWaitList&quot; ).click(function() {

                                                            if(isNaN($(&quot;#numMaxWaitList&quot;).val())){

                                                                $(&quot;#numMaxWaitList&quot;).val('0');
                                                            }
                                                            $(&quot;#numMaxWaitList&quot;).val(parseInt($(&quot;#numMaxWaitList&quot;).val())+1);
                                                       });

                                                       $( &quot;#btn_down_numMaxWaitList&quot; ).click(function() {

                                                           if(isNaN($(&quot;#numMaxWaitList&quot;).val())){

                                                                $(&quot;#numMaxWaitList&quot;).val('0');
                                                             }
                                                             if(parseInt($(&quot;#numMaxWaitList&quot;).val())-1 >=0){

                                                                 $(&quot;#numMaxWaitList&quot;).val(parseInt($(&quot;#numMaxWaitList&quot;).val())-1);
                                                             }  
                                                       });

                                                   

                                               
                                            
                                        
                                        
                                            PLUs
                                            
                                                
                                               
                                             
                                        
                                    
                                    
                                        
                                            Reserve Seats
                                            
                                                
                                                
                                             
                                                
                                                  
                                                  
                                                
                                                

                                                    $( &quot;#btn_up_NumRsvSeat&quot; ).click(function() {

                                                            if(isNaN($(&quot;#NumRsvSeat&quot;).val())){

                                                                $(&quot;#NumRsvSeat&quot;).val('0');
                                                              }
 
                                                           $(&quot;#NumRsvSeat&quot;).val(parseInt($(&quot;#NumRsvSeat&quot;).val())+1);
                                                           //$(&quot;#opt_NumRsvSeat&quot;).text(parseInt($(&quot;#opt_NumRsvSeat&quot;).val()));
                                                           //$('#NumRsvSeat option:eq(0)').prop('selected', true);

                                                    });

                                                    $( &quot;#btn_down_NumRsvSeat&quot; ).click(function() {

                                                        if(isNaN($(&quot;#NumRsvSeat&quot;).val())){

                                                            $(&quot;#NumRsvSeat&quot;).val('0');
                                                          }
                                                         if(parseInt($(&quot;#NumRsvSeat&quot;).val())-1 >=0){
                                                             
                                                             $(&quot;#NumRsvSeat&quot;).val(parseInt($(&quot;#NumRsvSeat&quot;).val())-1);
                                                             //$(&quot;#opt_NumRsvSeat&quot;).text(parseInt($(&quot;#opt_NumRsvSeat&quot;).val()));
                                                             //$('#NumRsvSeat option:eq(0)').prop('selected', true);
                                                         }  

                                                    });

                                                

                                            
                                            
                                        
                                        
                                            Seats Expire On
                                            
                                                
                                                
                                            
                                            
                                        
                                    
                                                                        
                                
                            
                            
                                
                                $( &quot;#NumUnlimitReg&quot; ).change(function() {
                                    
                                   $(&quot;#ChkAllowUnlimitReg&quot;).prop('checked', true);
                                   $( &quot;#ChkAllowUnlimitReg&quot; ).val(1);
                                    
                                });
                                
                                
                                $(&quot;#ChkAllowUnlimitReg&quot;).change(function() {
                                    var ischecked= $(this).is(':checked');
                                    if(ischecked){
                                        
                                        $(&quot;#2stcheckmark&quot;).show();
                                        $( &quot;#ChkAllowUnlimitReg&quot; ).val(1);
                                    }
                                    else{
                                        
                                        $(&quot;#2stcheckmark&quot;).hide();
                                        $( &quot;#ChkAllowUnlimitReg&quot; ).val(0);
                                    }
                                }); 

                             
                                 function btnRegRequire(val){
                                    
                                    if(val==&quot;2&quot;){
                                        
                                            $(&quot;#reg_require_approval&quot;).hide();
                                            $(&quot;#min_class_size&quot;).hide();
                                            $(&quot;#rbnRegReqAppr&quot;).val(2);

                                    }  
                                    else if(val==&quot;0&quot;){

                                          $(&quot;#reg_require_approval&quot;).show();   
                                          $(&quot;#min_class_size&quot;).hide();   
                                          $(&quot;#rbnRegReqAppr&quot;).val(0);

                                    }  
                                    else if(val==&quot;1&quot;){

                                            $(&quot;#min_class_size&quot;).show();
                                            $(&quot;#reg_require_approval&quot;).hide();
                                            $(&quot;#rbnRegReqAppr&quot;).val(1);

                                    }
                                  }
                                 
                                function revertTo(oldval){
                                    
                                     //btnRegRequire(oldval);
                                     
                                     
                                     if(oldval==&quot;2&quot;){
                                        
                                            $(&quot;#reg_require_approval&quot;).hide();
                                            $(&quot;#min_class_size&quot;).hide();
                                            $(&quot;#rbnRegReqAppr&quot;).val(2);

                                    }  
                                    else if(oldval==&quot;0&quot;){

                                          $(&quot;#reg_require_approval&quot;).show();   
                                          $(&quot;#min_class_size&quot;).hide();   
                                          $(&quot;#rbnRegReqAppr&quot;).val(0);

                                    }  
                                    else if(oldval==&quot;1&quot;){

                                            $(&quot;#min_class_size&quot;).show();
                                            $(&quot;#reg_require_approval&quot;).hide();
                                            $(&quot;#rbnRegReqAppr&quot;).val(1);

                                    }
                                
                                    if(oldval=='0'){
                                        
                                        //$(&quot;#RegistrationRequireApproval&quot;).trigger(&quot;click&quot;);
                                        $(&quot;#RegistrationRequireApproval&quot;).removeClass(&quot;selected&quot;);
                                        $(&quot;#AutoApprove&quot;).removeClass(&quot;selected&quot;);
                                        $(&quot;#rbnRegReqAppr&quot;).removeClass(&quot;selected&quot;);
                                        $(&quot;#min_enrollment_req&quot;).removeClass(&quot;selected&quot;);
                                        $(&quot;#RegistrationRequireApproval&quot;).addClass(&quot;selected&quot;);

                                    }
                                    else if(oldval=='1'){


                                       // $(&quot;#min_enrollment_req&quot;).trigger(&quot;click&quot;); 

                                        $(&quot;#AutoApprove&quot;).removeClass(&quot;selected&quot;);
                                        $(&quot;#rbnRegReqAppr&quot;).removeClass(&quot;selected&quot;);
                                        $(&quot;#min_enrollment_req&quot;).removeClass(&quot;selected&quot;);
                                        $(&quot;#RegistrationRequireApproval&quot;).removeClass(&quot;selected&quot;);

                                         $(&quot;#min_enrollment_req&quot;).addClass(&quot;selected&quot;);


                                    }
                                    else if(oldval=='2'){
                                        
                                      // $(&quot;#AutoApprove&quot;).trigger(&quot;click&quot;);

                                        $(&quot;#AutoApprove&quot;).removeClass(&quot;selected&quot;);
                                        $(&quot;#rbnRegReqAppr&quot;).removeClass(&quot;selected&quot;);
                                        $(&quot;#min_enrollment_req&quot;).removeClass(&quot;selected&quot;);
                                        $(&quot;#RegistrationRequireApproval&quot;).removeClass(&quot;selected&quot;);
                                        $(&quot;#AutoApprove&quot;).addClass(&quot;selected&quot;);

                                    }
                                    else{

                                        $(&quot;#min_enrollment_req&quot;).trigger(&quot;click&quot;); 

                                        $(&quot;#AutoApprove&quot;).removeClass(&quot;selected&quot;);
                                        $(&quot;#rbnRegReqAppr&quot;).removeClass(&quot;selected&quot;);
                                        $(&quot;#RegistrationRequireApproval&quot;).removeClass(&quot;selected&quot;);

                                         $(&quot;#min_enrollment_req&quot;).addClass(&quot;selected&quot;);

                                    }

                                    $(&quot;#ChkAllowUnlimitReg&quot;).trigger(&quot;change&quot;);
                                    
                                } 
                                
                                function btnRegRequir_editmode(val){
                                  
                                  var oldval=$(&quot;#rbnRegReqAppr&quot;).val();
                                  
                                  if(val==&quot;2&quot;){
                                      
                                                                             
                                          $(&quot;#reg_require_approval&quot;).hide();
                                          $(&quot;#min_class_size&quot;).hide();
                                          $(&quot;#rbnRegReqAppr&quot;).val(2);
                                          revertTo('2');
                                          
                                       
                                      
                                   
                                  }  
                                  else if(val==&quot;0&quot;){
                                      
                                                                                        
                                         $(&quot;#reg_require_approval&quot;).show();   
                                        $(&quot;#min_class_size&quot;).hide();   
                                        $(&quot;#rbnRegReqAppr&quot;).val(0);
                                        revertTo('0');
                                                                                
                                   
                                  }  
                                  else if(val==&quot;1&quot;){
                                      
                                                                                   $(&quot;#min_class_size&quot;).show();
                                           $(&quot;#reg_require_approval&quot;).hide();
                                           $(&quot;#rbnRegReqAppr&quot;).val(1);
                                           revertTo('1');
                                       
                                  }
                                }
                                
                                
                                btnRegRequire($(&quot;#rbnRegReqAppr&quot;).val());
                                
                                if(parseInt($(&quot;#rbnRegReqAppr&quot;).val())==0){
                                    
                                    //$(&quot;#RegistrationRequireApproval&quot;).trigger(&quot;click&quot;);
                                    $(&quot;#RegistrationRequireApproval&quot;).removeClass(&quot;selected&quot;);
                                    $(&quot;#AutoApprove&quot;).removeClass(&quot;selected&quot;);
                                    $(&quot;#rbnRegReqAppr&quot;).removeClass(&quot;selected&quot;);
                                    $(&quot;#RegistrationRequireApproval&quot;).addClass(&quot;selected&quot;);
                                    
                                }
                                else if(parseInt($(&quot;#rbnRegReqAppr&quot;).val())==1){
                                    
                                    
                                   // $(&quot;#min_enrollment_req&quot;).trigger(&quot;click&quot;); 
                                   
                                    $(&quot;#AutoApprove&quot;).removeClass(&quot;selected&quot;);
                                    $(&quot;#rbnRegReqAppr&quot;).removeClass(&quot;selected&quot;);
                                    $(&quot;#RegistrationRequireApproval&quot;).removeClass(&quot;selected&quot;);
                                    
                                     $(&quot;#min_enrollment_req&quot;).addClass(&quot;selected&quot;);
                                     
                                    
                                }
                                else if(parseInt($(&quot;#rbnRegReqAppr&quot;).val())==2){
                                    
                                  // $(&quot;#AutoApprove&quot;).trigger(&quot;click&quot;);
                                    
                                    $(&quot;#AutoApprove&quot;).removeClass(&quot;selected&quot;);
                                    $(&quot;#rbnRegReqAppr&quot;).removeClass(&quot;selected&quot;);
                                    $(&quot;#RegistrationRequireApproval&quot;).removeClass(&quot;selected&quot;);
                                    $(&quot;#AutoApprove&quot;).addClass(&quot;selected&quot;);
                                    
                                }
                                else{
                                    
                                    $(&quot;#min_enrollment_req&quot;).trigger(&quot;click&quot;); 
                                   
                                    $(&quot;#AutoApprove&quot;).removeClass(&quot;selected&quot;);
                                    $(&quot;#rbnRegReqAppr&quot;).removeClass(&quot;selected&quot;);
                                    $(&quot;#RegistrationRequireApproval&quot;).removeClass(&quot;selected&quot;);
                                    
                                     $(&quot;#min_enrollment_req&quot;).addClass(&quot;selected&quot;);
                                    
                                }
                                
                                $(&quot;#ChkAllowUnlimitReg&quot;).trigger(&quot;change&quot;);
                            
                        
                        
                        
                             $.validator.setDefaults({ ignore: '' });
                                 
                                var dynamicErrorMsg = function () { return &quot; Max PLUs allowed for selected component number is &quot; .concat($(&quot;#MaxCredits&quot;).val().toString()) } 
                                $.validator.addMethod(&quot;checkmaxcredit&quot;, function(value, element) {
                                    
                                    
                                     if(parseInt($(&quot;#txtCredits&quot;).val()) > $(&quot;#MaxCredits&quot;).val() &amp;&amp; $(&quot;#MaxCredits&quot;).val() != -1 ){
                                         
                                         return false;
                                     }
                                     else{
                                         
                                         return true
                                     }
                                        
                                        
                               }, dynamicErrorMsg );
                               
                                $.validator.addMethod(&quot;checkNumMinClassSize&quot;, function(value, element) {
                                    
                                      
                                     if(parseInt($(&quot;#numMinClassSize&quot;).val()) &lt;= 0 &amp;&amp; $(&quot;#rbnRegReqAppr&quot;).val()=='1'){
                                         
                                         return false;
                                     }
                                     else{
                                         
                                         return true
                                     }
                                        
                                        
                               }, &quot;Please enter a minimum enrollment.&quot;);
                               
                                $.validator.addMethod(&quot;EnrollRegMsgReq&quot;, function(value, element) {
                                    
                                     if($.trim($(&quot;#txtEnrollRegMsg&quot;).val())==''  &amp;&amp; $(&quot;#rbnRegReqAppr&quot;).val()=='0'){
                                         
                                         return false;
                                     }
                                     else{
                                         
                                         return true
                                     }
                                        
                                        
                               }, &quot;Please enter enrollment request notification.&quot;);
                               
                                $.validator.addMethod(&quot;checkDTRsvSeat&quot;, function(value, element) {
                                    
                                                                         
                                     if(parseInt($(&quot;#NumRsvSeat&quot;).val())>0){
                                         
                                         if($(&quot;#DTRsvSeat&quot;).val()!=''){
                                                
                                                var today = new Date();
                                                var dd = today.getDate();
                                                var mm = today.getMonth()+1; //January is 0!
                                                var yyyy = today.getFullYear();

                                                if(dd&lt;10) {
                                                    dd='0'+dd
                                                } 

                                                if(mm&lt;10) {
                                                    mm='0'+mm
                                                } 

                                                today = mm+'/'+dd+'/'+yyyy;
                                                ddate= new Date($(&quot;#DTRsvSeat&quot;).val());
                                                
                                                todateDate= new Date(today);
                                                if(ddate&lt;todateDate){
                                                    
                                                    return false;
                                                }
                                                else{
                                                    
                                                    return true;
                                                }
                                             
                                         }
                                         else{
                                             
                                             return true;
                                         }
                                         
                                     }
                                     else{
                                         
                                         return true;
                                     }
                                        
                                     
                                        
                               }, &quot;Reserved seat expiry date can not be less than today.&quot;);
                               
                               
                                $.validator.addMethod(&quot;checkSendEnrollEmailTo&quot;, function(value, element) {
                                    
                                    
                                     if(parseInt( $(&quot;#rbnRegReqAppr&quot;).val())==0 &amp;&amp; $(&quot;#txtSendEnrollEmailTo&quot;).val()==&quot;&quot;){
                                         
                                         return false;
                                     }
                                     else{
                                         
                                         return true
                                     }
                                        
                                        
                               }, &quot;Please enter Enroll Email&quot; );
                               
                                $.validator.addMethod(&quot;checkIfOneCheckboxChecked&quot;, function(value, element) {
                                    
                                    if(parseInt( $(&quot;#rbnRegReqAppr&quot;).val())==0){ 
                                        
                                        if($(&quot;#RequireWorkLocApproval&quot;).is(&quot;:checked&quot;) || $(&quot;#RequireDistrictApproval&quot;).is(&quot;:checked&quot;) ||  $(&quot;#RequireGlobalApproval&quot;).is(&quot;:checked&quot;)){
                                        
                                            return true;
                                        
                                        }
                                        else{
                                            
                                            return false;
                                        }
                                    }
                                    else{
                                        
                                        return false;
                                    }
                                        
                               }, &quot;Please select at least one approval.&quot; );
                               
                               jQuery.validator.addMethod(&quot;notEqual0&quot;, function(value, element, param) {
                                    if (( $('#rbnRegReqAppr').val()==&quot;0&quot; || $('#rbnRegReqAppr').val()==&quot;2&quot; || $('#rbnRegReqAppr').val()==&quot;1&quot;) &amp;&amp; value==0){
                                        
                                        return  false;
                                    }
                                    else{
                                        
                                        return true;
                                    }
                                  }, &quot;Maximum class size can not be 0.&quot;);
                                  
                                 jQuery.validator.addMethod(&quot;validate_numMaxSizeExternal&quot;, function(value, element, param) {
                                 
                                    
                                    if ($.trim($('#numMaxSizeExternal').val()).toLowerCase()==&quot;unlimited&quot; || $('#numMaxSizeExternal').val().match(/^\d+$/) ){
                                        
                                        return  true;
                                    }
                                    else{
                                        
                                        return false;
                                    }
                                  }, &quot;Please enter only digits.&quot;);
                                  
                                 jQuery.validator.addMethod(&quot;validate_numMaxWaitList&quot;, function(value, element, param) {
                                 
                                    
                                    if ($.trim($('#numMaxWaitList').val()).toLowerCase()==&quot;unlimited&quot; ||  $.trim($('#numMaxWaitList').val()).toLowerCase()==&quot;no wait list&quot; || $('#numMaxWaitList').val().match(/^\d+$/) ){
                                        
                                        return  true;
                                    }
                                    else{
                                        
                                        return false;
                                    }
                                  }, &quot;Please enter only digits.&quot;);
                                  
                                 jQuery.validator.addMethod(&quot;validate_NumUnlimitReg&quot;, function(value, element, param) {
                                 
                                    
                                    if ($.trim($('#NumUnlimitReg').val()).toLowerCase()==&quot;unlimited&quot; || $('#NumUnlimitReg').val().match(/^\d+$/) ){
                                        
                                        return  true;
                                    }
                                    else{
                                        
                                        return false;
                                    }
                                  }, &quot;Please enter only digits.&quot;);
                                  
                               $('#frm_limit').validate({

                                        rules: {
                                                        
                                                numMinClassSize:{
                                                     checkNumMinClassSize: {
                                                         
                                                         depends: function(element) {
                                                                return parseInt($(&quot;#rbnRegReqAppr&quot;).val())==1;
                                                            }
                                                     }
                                                    
                                                },
                                                numMaxClassSize:{
                                                  digits   :true,
                                                  notEqual0: true,
                                                  max:32767
                                                },        
                                                numClassRoomHours:{
                                                  digits   :true
                                                },        
                                                numNonClassRoomHours:{
                                                  digits   :true
                                                },        
                                                NumUnlimitReg:{
                                                  validate_NumUnlimitReg:{
                                                      
                                                       depends: function(element) {
                                                                return parseInt($(&quot;#rbnRegReqAppr&quot;).val())==0;
                                                            }
                                                  }
                                                },        
                                                NumRsvSeat:{
                                                  digits   :true
                                                },        
                                                NumRandDaysAfterReg:{
                                                  digits   :true
                                                },        
                                                numMinClassSize:{
                                                  digits   :true
                                                },        
                                                txtCredits:{
                                                  number :true
                                                },        
                                                numMaxSizeExternal:{
                                                   validate_numMaxSizeExternal:true
                                                },        
                                                numMaxWaitList:{
                                                    validate_numMaxWaitList:true
                                                },        
                                                /*txtSendEnrollEmailTo:{
                                                    
                                                    checkSendEnrollEmailTo:{
                                                        
                                                        depends: function(element) {
                                                                return parseInt($(&quot;#rbnRegReqAppr&quot;).val())==0;
                                                            }
                                                    },
                                                    email:{
                                                        
                                                         depends: function(element) {
                                                                return parseInt($(&quot;#rbnRegReqAppr&quot;).val())==0;
                                                            }
                                                    }
                                                },
                                                txtEnrollRegMsg:{
                                                    
                                                    EnrollRegMsgReq:{
                                                        
                                                         depends: function(element) {
                                                                return parseInt($(&quot;#rbnRegReqAppr&quot;).val())==0;
                                                            }
                                                    }
                                                },*/
    
                                             chkboxes:{
                                                    
                                                    checkIfOneCheckboxChecked:{
                                                        
                                                         depends: function(element) {
                                                                return parseInt($(&quot;#rbnRegReqAppr&quot;).val())==0;
                                                            }
                                                    }
                                                },
                                                DTRsvSeat:{
                                                    
                                                  checkDTRsvSeat:true  
                                                }
                                                       

                                        },
                                         errorPlacement: function(error, element) {
                                                error.appendTo( element.closest(&quot;.elementclass&quot;).next());
                                          },
                                          invalidHandler: function(e,validator) {
                                            //validator.errorList contains an array of objects, where each object has properties &quot;element&quot; and &quot;message&quot;.  element is the actual HTML Input.
                                         /*
                                           for (var i=0;i&lt;validator.errorList.length;i++){
                                                console.log(validator.errorList[i]);
                                            }

                                            //validator.errorMap is an object mapping input names -> error messages
                                            for (var i in validator.errorMap) {
                                              console.log(i, &quot;:&quot;, validator.errorMap[i]);
                                            }
                                            */
                                        }

                              });   
                        
                        
                    
                    
                        
                          
                            

                            Limits &amp; ScheduleFees
                           
                           
                               
                                   
                                       
                                            Non-Member Fee
                                           
                                               
                                               
                                           
                                            
                                       
                                   
                                   
                                       
                                            Member Fee
                                           
                                               
                                               
                                           
                                            
                                       
                                   
                                   
                                       
                                            Materials Fee
                                           
                                               
                                               
                                           
                                            
                                       
                                   
                               
                           
                            
                                $.validator.setDefaults({ ignore: '' });
                                 $('#frm_fee').validate({

                                       rules: {
                                               numNonEmpCostint: {
                                                       number: true

                                               },
                                               numEmployeeCostint: {
                                                       number: true

                                               },
                                               numMaterialCostint: {
                                                       number: true

                                               } 
                                       },
                                        errorPlacement: function(error, element) {
                                             error.appendTo( element.closest(&quot;.input-group&quot;).next());
                                         }

                                   });   
                           
                            
                    
                     
                         
                    
                         
                                

                            Limits &amp; ScheduleSchedule
                            
                            
                                
                                    
                                        
                                            Start Date &amp; Time*
                                            
                                                
                                                
                                            
                                            
                                        
                                        
                                            End Date &amp; Time*
                                            
                                                
                                                
                                            
                                            
                                        
                                    
                                    
                                        
                                            Repeat Interval*
                                            
                                                None/Irregular
                                                Daily
                                                Weekly
                                                Monthly same date
                                                Daily (Week Days Only)
                                            
                                        
                                        
                                            Week Days*
                                            
                                                 Sunday
                                                  Monday
                                                  Tuesday
                                                  Wednesday 
                                                  Thursday 
                                                  Friday 
                                                  Saturday 
                                            
                                        
                                    
                                     
                                    
                                    Schedule
                                    
                                    
                                    
                                    
                                        
                                        
                                    
                                    

                                       $(&quot;.fld_shedule&quot;).change(function(e) {
                                            
                                          $(&quot;#hdn_schedule_change&quot;).val(&quot;true&quot;)
                                       })
                                       $(&quot;#cboInterVal&quot;).change(function(e) {

                                           if($(this).val()=='2'){

                                              $(&quot;#li_day_week&quot;).show(); 
                                           }
                                           else{

                                                $(&quot;#li_day_week&quot;).hide(); 
                                           }
                                       })
                                       
                                       $(&quot;#cboInterVal&quot;).trigger('click');
                                       $('#btn_generate_date').click(function (e) {
                                            
                                            var flag=true;
                                            if($(&quot;#cboInterVal&quot;).val()=='2'){
                                                
                                                 if(!jQuery(&quot;input[name='chkDayOfWeek[]']:checked&quot;).length) {
                                                     
                                                     alert('Please select atleast one day of week.');
                                                     flag=false;
                                                 }
                                                 
                                                
                                            }
                                          
                                           if(flag){
                                               
                                                $(&quot;#is_date_generated&quot;).rules('remove', 'check_schedule_dates');
                                                $(&quot;#is_trening_year_set&quot;).rules('remove', 'greaterThanZero');
                                                hdn_schedule_change=$(&quot;#hdn_schedule_change&quot;).val();
                                                $('#frm_schedule').validate();
                                                if ($(&quot;#frm_schedule&quot;).valid()) {
                                                    
                                                    //waitingDialog.hide();
                                                    //waitingDialog.show('Loading....');
                                                    
                                                    e.preventDefault(); 
                                                    var myform = $('#frm_schedule');
                                                    var serialized = myform.serialize();
                                                    var datastring = serialized;
                                                     $.ajax({
                                                         type: &quot;POST&quot;,
                                                         cache: false,
                                                         timeout:60000,
                                                         url: &quot;/PGCTechnologyServices/course-manager/generatedays?MAXDATES_CAL=50&amp;sure=0&amp;sid=kcbtrojn&amp;classid=0&amp;btn_generate_date=&quot;+hdn_schedule_change+&quot;&amp;time=&quot;+new Date().getTime(),
                                                         data: datastring,
                                                         beforeSend:function() {
                                                            waitingDialog.hide();
                                                            $('#ajax_instr_loader').hide();
                                                            waitingDialog.show('Loading....');
                                                         }, 
                                                         success: function(data) {
                                                             
                                                             dateChange=true;
                                                             var n = data.indexOf(&quot;Are you sure you want to generate&quot;);
                                                             if(n!== -1){
                                                                 
                                                                 var r = confirm(data);
                                                                 if (r == true) {
                                                                     
                                                                   //waitingDialog.hide();  
                                                                   //waitingDialog.show('Loading....');
                                                                   
                                                                    hdn_schedule_change=$(&quot;#hdn_schedule_change&quot;).val();
                                                                    e.preventDefault(); 
                                                                    var myform = $('#frm_schedule');
                                                                    var serialized = myform.serialize();
                                                                    var datastring = serialized;
                                                                     $.ajax({
                                                                         type: &quot;POST&quot;,
                                                                         cache: false,
                                                                         timeout:60000,
                                                                         url: &quot;/PGCTechnologyServices/course-manager/generatedays?classid=0&amp;MAXDATES_CAL=50&amp;sure=1&amp;sid=kcbtrojn&amp;classid=0&amp;btn_generate_date=&quot;+hdn_schedule_change+&quot;&amp;time=&quot;+new Date().getTime(),
                                                                         data: datastring,
                                                                          beforeSend:function() {
                                                                            waitingDialog.hide();
                                                                            $('#ajax_instr_loader').hide();
                                                                            waitingDialog.show('Loading....');
                                                                         }, 
                                                                         success: function(data) {
                                                                             
                                                                              $.ajax({
                                                                                type: &quot;POST&quot;,
                                                                                cache: false,
                                                                                timeout:60000,
                                                                                url: &quot;/PGCTechnologyServices/course-manager/trainingyearset?classid=0&amp;sid=kcbtrojn&amp;classid=0&amp;btn_generate_date=&quot;+hdn_schedule_change+&quot;&amp;time=&quot;+new Date().getTime()+'&amp;tdate='+$(&quot;#generate_start_date&quot;).val()+'&amp;tedate='+$(&quot;#generate_end_date&quot;).val(),
                                                                                data: {},
                                                                                 beforeSend:function() {
                                                                                   waitingDialog.hide();
                                                                                   $('#ajax_instr_loader').hide();
                                                                                   waitingDialog.show('Loading....');
                                                                                }, 
                                                                                success: function(data_) {

                                                                                   $(&quot;#dates_body&quot;).html(data);  
                                                                                    dateChange=true;
                                                                                    //waitingDialog.hide();
                                                                                   
                                                                                    $('#ajax_instr_loader').hide();
                                                                                    $(&quot;#dates_body&quot;).html(data);  
                                                                                    $(&quot;#is_trening_year_set&quot;).val(data_);  
                                                                                    $(&quot;#is_trening_year_set&quot;).rules('add', 'greaterThanZero');
                                                                                      
                                                                                    showErrosIfAnySingle();
                                                                                    waitingDialog.hide();
                                                                                    
                                                                                    
                                                                               },     
                                                                               error: function (xhr, ajaxOptions, thrownError) {

                                                                                           waitingDialog.hide();
                                                                                           $('#ajax_instr_loader').hide();
                                                                                           //alert(xhr.status);
                                                                                           //alert(thrownError);
                                                                                           //waitingDialog.hide();
                                                                                    },
                                                                                    complete: function() {

                                                                                           waitingDialog.hide();
                                                                                           $('#ajax_instr_loader').hide();
                                                                                       }

                                                                                });
                                                                             
                                                                              
                                                                        },     
                                                                        error: function (xhr, ajaxOptions, thrownError) {
                                                                            
                                                                                    waitingDialog.hide();
                                                                                    $('#ajax_instr_loader').hide();
                                                                                    //alert(xhr.status);
                                                                                    //alert(thrownError);
                                                                                    //waitingDialog.hide();
                                                                             },
                                                                             complete: function() {
                                             
                                                                                    waitingDialog.hide();
                                                                                    $('#ajax_instr_loader').hide();
                                                                                }

                                                                         });
                                                                 
                                                                 }
                                                                 else{
                                                                       
                                                                      // jQuery('#generate_start_date').datetimepicker('hide');
                                                                      // jQuery('#generate_end_date').datetimepicker('hide');
                                                                       
                                                                       
                                                                    //waitingDialog.hide();
                                                                 }
                                                              }
                                                              else{
                                                             
                                                                   
                                                                    
                                                                      $.ajax({
                                                                                type: &quot;POST&quot;,
                                                                                cache: false,
                                                                                timeout:60000,
                                                                                url: &quot;/PGCTechnologyServices/course-manager/trainingyearset?classid=0&amp;sid=kcbtrojn&amp;classid=0&amp;btn_generate_date=&quot;+hdn_schedule_change+&quot;&amp;time=&quot;+new Date().getTime()+'&amp;tdate='+$(&quot;#generate_start_date&quot;).val()+'&amp;tedate='+$(&quot;#generate_end_date&quot;).val(),
                                                                                data: {},
                                                                                 beforeSend:function() {
                                                                                   waitingDialog.hide();
                                                                                   $('#ajax_instr_loader').hide();
                                                                                   waitingDialog.show('Loading....');
                                                                                }, 
                                                                                success: function(data_) {
                                                                                    
                                                                                    $(&quot;#dates_body&quot;).html(data);   
                                                                                     //waitingDialog.hide();
                                                                                      //waitingDialog.hide();
                                                                                    $('#ajax_instr_loader').hide();
                                                                                    $(&quot;#is_trening_year_set&quot;).val(data_);  
                                                                                     $(&quot;#is_trening_year_set&quot;).rules('add', 'greaterThanZero');
                                                                                   
                                                                                    showErrosIfAnySingle();
                                                                                    waitingDialog.hide();
                                                                                   
                                                                               },     
                                                                               error: function (xhr, ajaxOptions, thrownError) {

                                                                                           waitingDialog.hide();
                                                                                           $('#ajax_instr_loader').hide();
                                                                                           //alert(xhr.status);
                                                                                           //alert(thrownError);
                                                                                           //waitingDialog.hide();
                                                                                    },
                                                                                    complete: function() {

                                                                                           waitingDialog.hide();
                                                                                           $('#ajax_instr_loader').hide();
                                                                                       }

                                                                                });
                                                             
                                                                }
                                                            //console.log(data);
                                                             //var obj = jQuery.parseJSON(data); if the dataType is not specified as json uncomment this
                                                             // do what ever you want with the server response
                                                            showErrosIfAnySingle();
                                                            waitingDialog.hide();
                                                            $('#ajax_instr_loader').hide();

                                                         },
                                                          error: function (xhr, ajaxOptions, thrownError) {
                                                          
                                                             waitingDialog.hide();
                                                             $('#ajax_instr_loader').hide();
                                                             //alert(xhr.status);
                                                             //alert(thrownError);
                                                             //waitingDialog.hide();
                                                           },
                                                           complete: function() {
                                             
                                                                waitingDialog.hide();
                                                                $('#ajax_instr_loader').hide();
                                                            } 

                                                     });

                                                   $('#dv_course_date').show();
                                                   
                                                 }    
                                           }
                                           
                                            //$('#dv_course_date').show();
                                        });

                                        
                                
                                
                                    
                                    
                                    
                                        
                                            
                                                
                                                    
                                                          Add Course Date
                                                    
                                                
                                            
                                        
                                        
                                            
                                                
                                                    Date
                                                    Start Time
                                                    End Time
                                                    Action
                                                
                                            
                                            
                                                No Schedule 
                                            
                                        
                                        
                                    
                                
                            
                             
                                $.validator.setDefaults({ ignore: '' });

                                jQuery.validator.addMethod(&quot;greaterThan&quot;, 
                                function(value, element, params) {

                                    if (!/Invalid|NaN/.test(new Date(value))) {
                                        return new Date(value) > new Date($(params).val());
                                    }

                                    return isNaN(value) &amp;&amp; isNaN($(params).val()) 
                                        || (Number(value) > Number($(params).val())); 
                                },'End date must be greater than start date.');
                                
                                 $.validator.addMethod(&quot;greaterThanZero&quot;, function(value, element) {
                                     
                                    if(parseInt(value)==-1){
                                        
                                        
                                        return true;
                                    } 
                                    return this.optional(element) || (parseFloat(value) > 0);
                                 }, &quot;It would appear you are trying to schedule a class that ends in a training year that is not yet created. Please create it before scheduling the class.&quot;);

                                  $.validator.addMethod(&quot;check_schedule_dates&quot;, function(value, element) {

                                           
                                           if ($.trim($(&quot;#dates_body&quot;).text()) == 'No Schedule')
                                           {
                                               return false;
                                           }
                                           else
                                           {
                                               return true;
                                           }

                                  }, &quot;Please generate schedule&quot;);

                                 $('#frm_schedule').validate({

                                           rules: {
                                                   is_date_generated: {
                                                           check_schedule_dates: true

                                                   },
                                                    generate_end_date: { 
                                                            greaterThan: &quot;#generate_start_date&quot; 
                                                    }
                                                     

                                           },
                                            errorPlacement: function(error, element) {
                                                   error.appendTo( element.parent().next());
                                             }

                                       }); 
                                       
                                      
                                      
                                    
                                      $(&quot;#cboInterVal&quot;).change(function(){
                                          
                                          /*if($(&quot;#dates_body&quot;).text()!=&quot;No Schedule&quot;){
                                              
                                                var r = confirm(&quot;Are you sure you want to override previously generated dates?&quot;);
                                                if (r == true) {

                                                   $(&quot;#btn_generate_date&quot;).trigger('click'); 
                                                   dateChange=true;
                                                    
                                                }
                                            
                                             }*/
                                          
                                      })
                                      
                                      $(&quot;input[name='chkDayOfWeek[]']&quot;).click(function(){
                                          
                                          /*if($(&quot;#dates_body&quot;).text()!=&quot;No Schedule&quot;){
                                              
                                                var r = confirm(&quot;Are you sure you want to override previously generated dates?&quot;);
                                                if (r == true) {

                                                   $(&quot;#btn_generate_date&quot;).trigger('click'); 
                                                   dateChange=true;
                                                    
                                                }
                                                else{
                                                    
                                                     //jQuery('#generate_start_date').datetimepicker('show');
                                                     //jQuery('#generate_end_date').datetimepicker('hide');
                                                }
                                            
                                             }*/
                                          
                                      })
                                       
                            
                          
                    
                    
                         
                             
                        Limits &amp; ScheduleDeadlines
                        
                        
                            
                                
                                     
                                        Course Start Date
                                        
                                        
                                            
                                          
                                            
                                        
                                            
                                     
                                        Course End Date
                                        
                                        
                                            
                                          
                                            
                                        
                                        
                                        
                                            
                                    
                                
                                    
                                        Pace
                                        
                                            
                                                Normal
                                                Self Paced

                                            
                                         
                                        
                                    
                                    
                                        Publication Option
                                        
                                            
                                                Choose..
                                                                                                   Do not publish
                                                                                                   Make available to all registration
                                                                                                   Make available to Course Track Only
                                                                                            
                                        
                                        
                                        
                                            $(&quot;#cboPubOption&quot;).change(function(e) {
                                                  //waitingDialog.hide();
                                                  //waitingDialog.show('Loading....');
                                                            var selected=this.value;
                                                            $.ajax({
                                                               type: &quot;GET&quot;,
                                                               cache: false,
                                                               timeout:60000,
                                                               url: &quot;/PGCTechnologyServices/course-manager/puboptchange?cboPubOption=&quot;+this.value+&quot;&amp;ascDoNotPublish=01/01/2039&amp;sid=kcbtrojn&amp;time=&quot;+new Date().getTime() ,
                                                               dataType: &quot;json&quot;,
                                                                beforeSend:function() {
                                                                    waitingDialog.hide();
                                                                    $('#ajax_instr_loader').hide();
                                                                    waitingDialog.show('Loading....');
                                                                 }, 
                                                               success: function(data) {
                                                                   
                                                                   if(selected==&quot;3&quot;){
                                                                       
                                                                      $(&quot;#internal_registration_start_date&quot;).val(data.dtInternalRegSDate); 
                                                                      $(&quot;#internal_registration_start_date&quot;).prop( &quot;disabled&quot;, true );
                                                                      $(&quot;#external_registration_start_date&quot;).val(data.dtExternalRegSDate); 
                                                                      $(&quot;#external_registration_start_date&quot;).prop( &quot;disabled&quot;, true );

                                                                   } 
                                                                   else{
                                                                         
                                                                         $(&quot;#external_registration_start_date&quot;).prop( &quot;disabled&quot;, false );
                                                                         $(&quot;#internal_registration_start_date&quot;).prop( &quot;disabled&quot;, false );
                                                                         $(&quot;#internal_registration_start_date&quot;).val(data.dtInternalRegSDate);
                                                                         $(&quot;#external_registration_start_date&quot;).val(data.dtExternalRegSDate); 
                                                                       
                                                                   }
                                                                   //waitingDialog.hide();
                                                                   //var obj = jQuery.parseJSON(data); if the dataType is not specified as json uncomment this
                                                                   // do what ever you want with the server response

                                                                  waitingDialog.hide();
                                                                  $('#ajax_instr_loader').hide();

                                                               },
                                                                error: function (xhr, ajaxOptions, thrownError) {
                                                                   
                                                                   waitingDialog.hide();
                                                                   $('#ajax_instr_loader').hide();
                                                                   //alert(xhr.status);
                                                                   //alert(thrownError);
                                                                   //waitingDialog.hide();
                                                                 },
                                                                 complete: function() {
                                             
                                                                    waitingDialog.hide();
                                                                    $('#ajax_instr_loader').hide();
                                                                }

                                                           });
                                                
                                             });
                                        
                                    
                                
                            
                            
                                
                                    
                                        Internal Registration Start Date &amp; Time*
                                        
                                            
                                            
                                        
                                        
                                    
                                    
                                        External Registration Start Date &amp; Time*
                                        
                                            
                                            
                                        
                                        
                                    
                                
                            
                            
                                
                                    
                                        Registration Deadline*
                                        
                                            
                                            
                                        
                                        
                                    
                                    
                                        Withdrawal Deadline*
                                        
                                            
                                            
                                        
                                        
                                    
                                
                            
                             
                                
                                    
                                        Evaluation Completion Date
                                        
                                            
                                            
                                        
                                        
                                    
                                   
                                
                            
                            
                        
                        
                                $.validator.setDefaults({ ignore: '' });
                                $.validator.addMethod(&quot;checkFollowupCompDate&quot;, function(value, element) {
                                  var dtFollowupCompDate=new Date($(&quot;#dtFollowupCompDate&quot;).val());
                                  
                                  var str=$(&quot;#generate_end_date&quot;).val();
                                  var res = str.split(&quot; &quot;);
                                  var endDate=new Date(res[0]);
                                  
                                  if(dtFollowupCompDate&lt;endDate){
                                      
                                      return false;
                                  }
                                  else{
                                      
                                      return true;
                                  }
                                        
                               }, &quot;Follow-up Completion date cannot be earlier than Course end date.&quot;);  
                               
                                $.validator.addMethod(&quot;checkInternalRegStartDate&quot;, function(value, element) {
                                  var dtInternalRegSDate=new Date($(&quot;#internal_registration_start_date&quot;).val());
                                  var endDate=new Date($(&quot;#generate_end_date&quot;).val());
                                  
                                  if(dtInternalRegSDate>endDate &amp;&amp; $(&quot;#cboPubOption&quot;).val()!=&quot;3&quot;){
                                      
                                      return false;
                                      
                                  }
                                  else{
                                      
                                      return true;
                                  }
                                        
                               }, &quot;Internal registration start cannot be later than Course end date.&quot;);  
                               
                                $.validator.addMethod(&quot;checkExternalRegStartDate&quot;, function(value, element) {
                                  var dtExternalRegSDate=new Date($(&quot;#external_registration_start_date&quot;).val());
                                  var startdate=new Date($(&quot;#generate_start_date&quot;).val());
                                  
                                  if(dtExternalRegSDate>startdate &amp;&amp; $(&quot;#cboPubOption&quot;).val()!=&quot;3&quot;){
                                      
                                      return false;
                                      
                                  }
                                  else{
                                      
                                      return true;
                                  }
                                        
                               }, &quot;External registration start cannot be later than Course start date.&quot;);  
                               
                                $.validator.addMethod(&quot;checkRegDeadline&quot;, function(value, element) {
                                  var dtRegDeadline=new Date($(&quot;#registration_deadline&quot;).val());
                                  var endDate=new Date($(&quot;#generate_end_date&quot;).val());
                                  
                                  if(dtRegDeadline>endDate){
                                      
                                      return false;
                                      
                                  }
                                  else{
                                      
                                      return true;
                                  }
                                        
                               }, &quot;Registration deadline cannot be later than Course end date.&quot;);  
                               
                                $.validator.addMethod(&quot;checkWithdrawDeadLine&quot;, function(value, element) {
                                  var dtWithdrawDeadLine=new Date($(&quot;#withdrawal_deadline&quot;).val());
                                  var endDate=new Date($(&quot;#generate_end_date&quot;).val());
                                  
                                  if(dtWithdrawDeadLine>endDate){
                                      
                                      return false;
                                      
                                  }
                                  else{
                                      
                                      return true;
                                  }
                                        
                               }, &quot;Withdrawal deadline cannot be later than Course end date.&quot;);  
                               
                               $('#frm_DeadLines').validate({
                                           onfocusout: false, 
                                           rules: {
                                                
                                                 cboPace: {
                                                       required: true

                                               },
                                                 cboPubOption: {
                                                       required: true

                                               },
                                                 dtInternalRegSDate: {
                                                       required: true,
                                                       checkInternalRegStartDate:true

                                               },
                                                 ExternalRegSDate: {
                                                       required: true,
                                                       checkExternalRegStartDate:true

                                               },
                                                 dtRegDeadline: {
                                                       required: true,
                                                       checkRegDeadline:true

                                               },
                                                 dtWithdrawDeadLine: {
                                                       required: true,
                                                       checkWithdrawDeadLine:true

                                               },
                                                 dtFollowupCompDate: {
                                                       required: true,
                                                       checkFollowupCompDate:true

                                               },

                                           },
                                            errorPlacement: function(error, element) {
                                                   error.appendTo( element.parent().next());
                                             }

                                       }); 
                        
                         
                    
                    
                        
                           
                          
                           RestrictionsEnrollment Restrictions
                          
                            
                            
                                
                                    
                                        
                                            
                                                  Add Restriction
                                            
                                        
                                    
                                
                                
                                    
                                        
                                            Group Name
                                            Group Limit
                                            Remove After
                                            Action
                                        
                                    
                                    
                                        No Groups                                    
                                
                            
                        
                             
                    
                                            
                              
                                 

                               RestrictionsTarget Audience
                              
                              
                                  
                                      
                                          
                                              
                                                  
                                                        Target Audience
                                                  
                                              
                                          
                                      
                                      
                                          
                                              
                                                  Target Audience
                                                  Delete
                                              
                                          
                                          
                                              No Target Audience                                          
                                      
                                      
                                      
                                  
                              
                                
                                         $.validator.setDefaults({ ignore: '' });

                                        $.validator.addMethod(&quot;checkTargetaudi&quot;, function(value, element) {


                                                 if ($.trim($(&quot;#tbl_targetAudinace&quot;).text()) == 'No Target Audience')
                                                 {
                                                     return false;
                                                 }
                                                 else
                                                 {
                                                     return true;
                                                 }

                                        }, &quot;Please add at least one target audience.&quot;);

                                       $('#frm_target_aud').validate({

                                                 rules: {
                                                         target_aud_added: {
                                                                 checkTargetaudi: true

                                                         }

                                                 },
                                                  errorPlacement: function(error, element) {
                                                         error.appendTo( element.next());
                                                   }

                                             });   
                                  
                               
                        
                                                                
                          
                                         
                                 RestrictionsRubrics
                                
                                
                                    
                                        
                                            
                                                
                                                    
                                                          Rubric 
                                                    
                                                
                                            
                                        
                                        
                                            
                                                
                                                                                                            Rubric
                                                        Domain
                                                        Component
                                                                                                        Delete
                                                
                                            
                                            
                                                No Rubrics                                            
                                        
                                        
                                    
                                
                        
                    
                                            
                        
                            
                                              
                                 RestrictionsCourse Tracks
                                
                                
                                    
                                        Associated Course Tracks
                                    
                                
                                
                                    
                                        
                                            

                                                No Associated Course Tracks Available

                                            
                                        
                                    
                                
                            
                                function loadTracks(){

                                       var class_id=0;
                                                                              $.ajax({
                                        type: &quot;GET&quot;,
                                        cache: false,
                                        timeout:60000,
                                        url: &quot;/PGCTechnologyServices/course-manager/gettracks?class_id=&quot;+class_id+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime(),
                                         beforeSend:function() {

                                            waitingDialog.hide();
                                            $('#ajax_instr_loader').hide();
                                            waitingDialog.show('Loading....');
                                         }, 
                                        success: function(data) {

                                            $(&quot;#tracks_body&quot;).html(data);
                                            //var obj = jQuery.parseJSON(data); if the dataType is not specified as json uncomment this
                                            // do what ever you want with the server response


                                           //waitingDialog.hide();
                                           showErrosIfAnySingle();
                                           waitingDialog.hide();
                                           $('#ajax_instr_loader').hide();
                                        },
                                         error: function (xhr, ajaxOptions, thrownError) {
                                            //alert(xhr.status);
                                            //alert(thrownError);
                                            //waitingDialog.hide();
                                          },
                                          complete: function() {

                                            waitingDialog.hide();
                                            $('#ajax_instr_loader').hide();
                                          }

                                    });

                                }
                            
                           
                        
                                                              
                        
                             
                         EvaluationSurvey Questions
                        
                        
                            
                                
                                    
                                        
                                            
                                                  Add Question
                                            
                                            
                                                  Insert Question
                                            
                                        
                                    
                                
                                
                                    
                                        
                                            Question
                                            Action
                                        
                                    
                                    
                                        No Questions 
                                    
                                
                            
                        
                        
                       
                                                              
                        
                             
                         EvaluationFollow-up Questions
                        
                        
                            
                                
                                    
                                        
                                            
                                                  Add Question
                                            
                                            
                                                  Insert Question
                                            
                                        
                                    
                                
                                
                                    
                                        
                                            Question
                                            Action
                                        
                                    
                                    
                                        No Questions 
                                    
                                
                            
                        
                        
                       
                     
                    
                    
                        var numbers = $(&quot;.list-group a&quot;).map(function(){
                            return parseFloat(this.getAttribute('tabindex')) || -Infinity;
                        }).toArray();

                        var maxTabs=Math.max.apply(Math, numbers);
                        
                    
                        
                        Previous
                        Next
                        Finish
                    
                

            
        
    


    
    
        
            
                Focus Area Information
            
              
                    
                    
                        Focus Area*
                        
                                
                                    Choose..
                                                                                 Certification
                                                                                 Code of Conduct
                                                                                 Curriculum and Instruction
                                                                                 ELL Education
                                                                                 English - Language Arts
                                                                                 ESOL Endorsement
                                                                                 General
                                                                                 GG
                                                                                 GG 01
                                                                                 Gifted Education
                                                                                 Gifted Endorsement
                                                                                 Job-Alike
                                                                                 Leadership
                                                                                 Literacy/Reading 
                                                                                 Pyramid of Intervention - Tier 1
                                                                                 Pyramid of Intervention - Tier 2
                                                                                 Pyramid of Intervention - Tier 3
                                                                                 Pyramid of Intervention - Tier 4
                                                                                 Reading Endorsement
                                                                                 School Counseling
                                                                                 School Graduation
                                                                                 School Healthcare
                                                                                 School Improvement
                                                                                 School Safety
                                                                                 Science
                                                                                 Singapore Math (Hall County)
                                                                                 Social Studies
                                                                                 Special Education
                                                                                 Technology
                                                                     
                               
                        
                         
                    
                       
                    
                        
                        
                        Submit
                    
                
              
            
                $.validator.setDefaults({ ignore: '' });
                  
                   $(&quot;#CboFocusArea&quot;).change(function(e) {

                       $(&quot;#txt_focusarea_text&quot;).val($(&quot;#CboFocusArea option:selected&quot;).text());
                     });
             
                  $('#frm_focusarea_add').validate({

                           rules: {
                                   CboFocusArea: {
                                           required: true

                                   },
                                   txtfocusperc:{
                                        required: true,
                                        digits:true

                                   }

                           },
                            errorPlacement: function(error, element) {
                                   error.appendTo( element.parent().next());
                             }

                       });   
                       
                       function resetAddFocusForm(){
                           
                            $('#CboFocusArea').prop('selectedIndex',0);
                            $('#txtfocusperc').val('');
                            $('#txt_focusarea_text').val('');
                            $(&quot;#CboFocusArea&quot;).removeAttr('disabled');
                       }
                       
                        $('.table_body').on('click', '.btn_delete_focus', function(e) { 
                           
                            e.preventDefault();
                            var name=$(this).attr('data-focusname');
                            var id=$(this).attr('data-focusid');
                            var r = confirm(&quot;Are you sure you want to delete &quot;+name+ &quot;?&quot;);
                            if (r == true) {
                                    //waitingDialog.hide();
                                    //waitingDialog.show('Loading....');
                                     $.ajax({
                                         type: &quot;GET&quot;,
                                         cache: false,
                                         timeout:60000,
                                         url: &quot;/PGCTechnologyServices/course-manager/deletefocusarea?focus_id=&quot;+id+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime(),
                                          beforeSend:function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                             waitingDialog.show('Loading....');
                                             
                                          }, 
                                         success: function(data) {

                                             $(&quot;#focus_area_body&quot;).html(data);
                                             //waitingDialog.hide();
                                               showErrosIfAnySingle(); 
                                               waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();

                                         },
                                          error: function (xhr, ajaxOptions, thrownError) {
                                              
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                             //alert(xhr.status);
                                             //alert(thrownError);
                                             //waitingDialog.hide();
                                           },
                                          complete: function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                         }

                                     });

                            }



                        });
                       $('.table_body').on('click', '.btn_edit_focus', function(e) { 
                           
                           e.preventDefault();
                           $(&quot;#btn_add_focus_modal&quot;).trigger('click');
                           $(&quot;#focus_action&quot;).val('edit');
                           var focusid=$(this).attr('data-focusid');
                           var focusvalue=$(this).attr('data-focusvalue');
                           $(&quot;#CboFocusArea&quot;).val(focusid);
                           $(&quot;#txtfocusperc&quot;).val(focusvalue);
                           $(&quot;#CboFocusArea&quot;).attr('disabled','disabled');
                           
                        });
                        
                        $(&quot;#btn_add_focus_modal&quot;).click(function(e) {
                             $(&quot;#CboFocusArea&quot;).removeAttr('disabled');
                             $(&quot;#focus_action&quot;).val('add');
                            
                             
                        });
                        
                       $(&quot;#btn_add_focusarea&quot;).click(function(e) {
                 
                            $(&quot;#CboFocusArea&quot;).removeAttr('disabled');
                            e.preventDefault(); 
                            var myform = $('#frm_focusarea_add');
                            var serialized = myform.serialize();

                    
                             if ($(&quot;#frm_focusarea_add&quot;).valid()) {

                                    //waitingDialog.hide();
                                     //waitingDialog.show('Loading....');
                                     var datastring = serialized;
                                      $.ajax({
                                          type: &quot;POST&quot;,
                                          cache: false,
                                          timeout:60000,
                                          url: &quot;/PGCTechnologyServices/course-manager/addfocusarea?sid=kcbtrojn&amp;time=&quot;+new Date().getTime(),
                                          data: datastring,
                                           beforeSend:function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                             waitingDialog.show('Loading....');
                                          }, 

                                          success: function(data) {
                                              
                                              data=jQuery.parseJSON(data);
                                              if(data.type=='success'){
                                                  
                                                   $(&quot;#focus_area_body&quot;).html(data.html);
                                                  
                                              } 
                                              else{
                                                  
                                                  alert(data.message);
                                                  
                                              }
                                             
                                              resetAddFocusForm();
                                              var magnificPopup = $.magnificPopup.instance; 
                                             // save instance in magnificPopup variable
                                              magnificPopup.close(); 
                                              //waitingDialog.hide();
                                              //var obj = jQuery.parseJSON(data); if the dataType is not specified as json uncomment this
                                              // do what ever you want with the server response
                                            showErrosIfAnySingle();  
                                            waitingDialog.hide();
                                            $('#ajax_instr_loader').hide();  
                                            
                                          },
                                           error: function (xhr, ajaxOptions, thrownError) {
                                               
                                              waitingDialog.hide();
                                             $('#ajax_instr_loader').hide(); 
                                              //alert(xhr.status);
                                              //alert(thrownError);
                                              //waitingDialog.hide();
                                           },
                                            complete: function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                         }

                                      });

                                }
                      });
                       
            
        
    
    
    
    
        
            
                Course Period Properties
            
              
                
                
                    Date*
                    
                        
                            
                            
                        
                        
                    
                
                
                    Start Time*
                    
                        
                            
                            
                        
                        
                    
                
                
                    End Time*
                    
                        
                            
                            
                        
                        
                    
                
                
                    
                    
                    Submit
                
            
               
            
                $.validator.setDefaults({ ignore: '' });
             
                 jQuery.validator.addMethod(&quot;compare_time&quot;, 
                    function(value, element, params) {
                        startdate=$(&quot;#course_period_date&quot;).val()+' '+$(&quot;#course_start_time&quot;).val();
                        enddate=$(&quot;#course_period_date&quot;).val()+' '+$(&quot;#course_end_time&quot;).val();
                        
                        
                          if (!/Invalid|NaN/.test(new Date(startdate))) {
                                        return new Date(startdate) &lt; new Date(enddate);
                                    }

                          return isNaN(startdate) &amp;&amp; isNaN(enddate) || (Number(startdate) &lt; Number(enddate)); 
                                
                       
                      
                    },'Please enter valid start and end time');
                                
                $('#frm_add_date').validate({

                         rules: {
                                 course_period_date: {
                                         required: true,
                                         date:true

                                 },
                                 course_start_time:{
                                      required: true,
                                 },
                                 course_end_time:{
                                      required: true,
                                 },
                                  course_end_time: { compare_time: true }

                         },
                          errorPlacement: function(error, element) {
                                   error.appendTo( element.parent().next());
                           }

                     });   
                     
                     function resetAddDateForm(){
                         
                         $(&quot;#course_period_date&quot;).val('');
                         $(&quot;#course_start_time&quot;).val('');
                         $(&quot;#course_end_time&quot;).val('');
                         $(&quot;#action_id&quot;).val('');
                         
                     }
                     
                     
                   
                     
                      $('.table_body').on('click', '.delete_date', function(e) { 
                           
                            e.preventDefault();
                            var id=$(this).attr('data-id');
                            var r = confirm(&quot;Are you sure you want to delete this date?&quot;);
                            if (r == true) {

                                    //waitingDialog.hide();
                                    //waitingDialog.show('Loading....');
                                     $.ajax({
                                         type: &quot;GET&quot;,
                                         cache: false,
                                         timeout:60000,
                                         url: &quot;/PGCTechnologyServices/course-manager/deletedate?date_id=&quot;+id+'&amp;sid=kcbtrojn&amp;time'+new Date().getTime(),
                                         beforeSend:function() {
                                             
                                            waitingDialog.hide();
                                            $('#ajax_instr_loader').hide();
                                            waitingDialog.show('Loading....');
                                         }, 
                                         success: function(data) {

                                                data=jQuery.parseJSON(data);
                                                $(&quot;#dates_body&quot;).html(data.schedule_tab_html);
                                                if($(&quot;#generate_start_date&quot;).val()!=data.generate_start_date || $(&quot;#generate_end_date&quot;).val()!=data.generate_end_date){
                                                       
                                                            $(&quot;#generate_start_date&quot;).val(data.generate_start_date);
                                                            $(&quot;#generate_end_date&quot;).val(data.generate_end_date);
                                                             /*var r = confirm(&quot;Are you sure you want to override previously generated dates?&quot;);
                                                            if (r == true) {

                                                               $(&quot;#btn_generate_date&quot;).trigger('click'); 
                                                               dateChange=true;

                                                            }
                                                            else{
                                                                return false;
                                                            }*/
                                                       
                                                       
                                                   }
                                                   else{
                                                       
                                                        $(&quot;#generate_start_date&quot;).val(data.generate_start_date);
                                                        $(&quot;#generate_end_date&quot;).val(data.generate_end_date);
                                                   }
                                                   
                                             //waitingDialog.hide();
                                             showErrosIfAnySingle();
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();

                                         },
                                          error: function (xhr, ajaxOptions, thrownError) {
                                              
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide(); 
                                             //alert(xhr.status);
                                             //alert(thrownError);
                                             //waitingDialog.hide();
                                           },
                                          complete: function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                         }

                                     });

                            }



                        });
                        
                     
                     $('.table_body').on('click', '.edit_date', function(e) { 
                           
                           e.preventDefault();
                           $(&quot;#btn-add-course-date&quot;).trigger('click');
                           $(&quot;#date_action&quot;).val('edit');
                           var dateid=$(this).attr('data-id');
                           var datecourse_period_date=$(this).attr('data-classdate');
                           var course_start_time=$(this).attr('data-starttime');
                           var course_start_time=$(this).attr('data-starttime');
                           var course_end_time=$(this).attr('data-endtime');
                           
                           $(&quot;#action_id&quot;).val(dateid);
                           $(&quot;#course_period_date&quot;).val(datecourse_period_date);
                           $(&quot;#course_start_time&quot;).val(course_start_time);
                           $(&quot;#course_end_time&quot;).val(course_end_time);
                           
                           
                        });
                        
                      $(&quot;#btn_add_date&quot;).click(function(e) {
                 
                            e.preventDefault(); 
                            var myform = $('#frm_add_date');
                            var serialized = myform.serialize();
                            $(&quot;#date_action&quot;).val('add');

                    
                             if ($(&quot;#frm_add_date&quot;).valid()) {

                                     //waitingDialog.hide();
                                     //waitingDialog.show('Loading....');
                                     var datastring = serialized;
                                      $.ajax({
                                          type: &quot;POST&quot;,
                                          cache: false,
                                          timeout:60000,
                                          url: &quot;/PGCTechnologyServices/course-manager/adddate?sid=kcbtrojn&amp;time=&quot;+new Date().getTime(),
                                          data: datastring,
                                           beforeSend:function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                             waitingDialog.show('Loading....');
                                          }, 
                                          success: function(data) {
                                              
                                                   data=jQuery.parseJSON(data);
                                              
                                                   $(&quot;#dates_body&quot;).html(data.schedule_tab_html);
                                                   
                                                   if($(&quot;#generate_start_date&quot;).val()!=data.generate_start_date || $(&quot;#generate_end_date&quot;).val()!=data.generate_end_date){
                                                       
                                                            $(&quot;#generate_start_date&quot;).val(data.generate_start_date);
                                                            $(&quot;#generate_end_date&quot;).val(data.generate_end_date);
                                                            /*var r = confirm(&quot;Are you sure you want to override previously generated dates?&quot;);
                                                            if (r == true) {

                                                               $(&quot;#btn_generate_date&quot;).trigger('click'); 
                                                               dateChange=true;

                                                            }
                                                            else{

                                                                return false;
                                                            }
                                                         */
                                                       
                                                   }
                                                   else{
                                                       
                                                        $(&quot;#generate_start_date&quot;).val(data.generate_start_date);
                                                        $(&quot;#generate_end_date&quot;).val(data.generate_end_date);
                                                   }
                                                  
                                                  
                                              
                                              resetAddDateForm();
                                              var magnificPopup = $.magnificPopup.instance; 
                                             // save instance in magnificPopup variable
                                              magnificPopup.close(); 
                                              //waitingDialog.hide();
                                              //var obj = jQuery.parseJSON(data); if the dataType is not specified as json uncomment this
                                              // do what ever you want with the server response
                                             showErrosIfAnySingle(); 
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                          },
                                           error: function (xhr, ajaxOptions, thrownError) {
                                               
                                              waitingDialog.hide();
                                              $('#ajax_instr_loader').hide();  
                                              //alert(xhr.status);
                                              //alert(thrownError);
                                              //waitingDialog.hide();
                                            },
                                           complete: function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                         }         

                                      });

                                }
                      });
                     
            
        
    
    
    
    
        
            
                Target Audience Options
            
             
                
                    
                                              Paraprofessionals
                                              System Level Administrators
                                              Substitute Teachers
                                              Bus Drivers
                                              All
                                              Graduation Project Leadership Team
                                              Graduation Coaches
                                              Academic Coaches
                                              General Ed Teachers
                                              Special Education Teachers
                                              High School Teachers
                                              Elementary School Teachers
                                              Middle School Teachers
                                              Technology
                                              GAVS Staff
                                              AP/IB High School Teachers
                                              School Counselors
                                              School Nurses
                                              School P.E. Coaches
                                              School Social Worker
                                              Elementary School Counselors
                                              Middle School Principals
                                              High School Counselors
                                              Elementary School Principals
                                              Middle School Counselors
                                              High School Principals
                                              Elementary School Vice Principals
                                              Middle School Vice Principals
                                              High School Vice Principals
                                              Media Specialist
                                              Science Teachers
                                          
                    Note: Hold down Ctrl key to select multiple.
                    
                        Submit
                    
                
             
            
                   $.validator.setDefaults({ ignore: '' });
                
                        $('#frm_add_target_aud').validate({

                            rules: {
                                    TAudience1: {
                                            required: true

                                    }


                            },
                             errorPlacement: function(error, element) {
                                   error.appendTo( element.next());
                              }

                        }); 
                        
                        function resetTragetAudForm(){
                            
                            $(&quot;#TAudience1 option:selected&quot;).prop(&quot;selected&quot;, false);
                            
                        }
                        
                        
                        
                        
                          $('.table_body').on('click', '.delete_traget_audi', function(e) { 
                           
                            e.preventDefault();
                            var name=$(this).attr('data-name');
                            var id=$(this).attr('data-id');
                            var r = confirm(&quot;Are you sure you want to delete &quot;+name+ &quot;?&quot;);
                            if (r == true) {
                                    //waitingDialog.hide();
                                    //waitingDialog.show('Loading....');
                                     $.ajax({
                                         type: &quot;GET&quot;,
                                         cache: false,
                                         timeout:60000,
                                         url: &quot;/PGCTechnologyServices/course-manager/deletetargetaudience?id=&quot;+id+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime(),
                                          beforeSend:function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                             waitingDialog.show('Loading....');
                                          }, 
                                         success: function(data) {

                                             data=jQuery.parseJSON(data);
                                             $(&quot;#tbl_targetAudinace&quot;).html(data.target_audiance_tab_html);
                                             //waitingDialog.hide();
                                              showErrosIfAnySingle();
                                              waitingDialog.hide();
                                              $('#ajax_instr_loader').hide();
                                                
                                         },
                                          error: function (xhr, ajaxOptions, thrownError) {
                                              
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();  
                                             //alert(xhr.status);
                                             //alert(thrownError);
                                             //waitingDialog.hide();
                                           },
                                           complete: function() {
                                             
                                                waitingDialog.hide();
                                                $('#ajax_instr_loader').hide();
                                            }
                                                   

                                     });

                            }



                        });
                        
                         $(&quot;#btn_add_traget_aud_&quot;).click(function(e) {
                            e.preventDefault(); 
                            var myform = $('#frm_add_target_aud');
                            var serialized = myform.serialize();

                    
                             if ($(&quot;#frm_add_target_aud&quot;).valid()) {

                                    //waitingDialog.hide();
                                     //waitingDialog.show('Loading....');
                                     
                                     var datastring = serialized;
                                      $.ajax({
                                          type: &quot;POST&quot;,
                                          cache: false,
                                          timeout:60000,
                                          url: &quot;/PGCTechnologyServices/course-manager/addtargetaudience?sid=kcbtrojn&amp;time=&quot;+new Date().getTime(),
                                          data: datastring,
                                           beforeSend:function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                             waitingDialog.show('Loading....');
                                          },
                                          success: function(data) {
                                              
                                              data=jQuery.parseJSON(data);
                                              
                                              if(data.error==&quot;true&quot;){
                                                  
                                                  alert(data.message);
                                              }
                                              else{
                                                    $(&quot;#tbl_targetAudinace&quot;).html(data.target_audiance_tab_html);
                                               }
                                             
                                              resetTragetAudForm();
                                              var magnificPopup = $.magnificPopup.instance; 
                                             // save instance in magnificPopup variable
                                              magnificPopup.close(); 
                                              //waitingDialog.hide();
                                              //var obj = jQuery.parseJSON(data); if the dataType is not specified as json uncomment this
                                              // do what ever you want with the server response
                                              showErrosIfAnySingle();
                                              waitingDialog.hide();
                                              $('#ajax_instr_loader').hide();
                                          },
                                           error: function (xhr, ajaxOptions, thrownError) {
                                               
                                              waitingDialog.hide();
                                              $('#ajax_instr_loader').hide(); 
                                              //alert(xhr.status);
                                              //alert(thrownError);
                                              //waitingDialog.hide();
                                            },
                                            complete: function() {
                                             
                                                waitingDialog.hide();
                                                $('#ajax_instr_loader').hide();
                                            }

                                      });

                                }
                      });
                        
            
        
    
    
    
    
        
            
                Enrollment Group Limit
            
           

            
                
                    Group Name*
                    
                        
                        
                    
                    
                
                
                    Group Limit *
                    
                        
                    
                    
                
                
                    Remove Limit After
                    
                        
                            
                            
                        
                        
                    
                
                
                    
                    
                    Submit
                
            
              
            
          
              
             
              
               $('#txtClassEnrollGroupName').bootcomplete({
                url:'/PGCTechnologyServices/course-manager/getclassgroups',
                'idField' :true,
                callback: function() {
                  
                 // GroupChange($(&quot;#txtClassEnrollGroupName&quot;).val());
                }
                
            });
            
                $.validator.setDefaults({ ignore: '' });
                
                  $('#frm_restriction_add').validate({

                           rules: {
                                   txtClassEnrollGroupName: {
                                           required: true

                                   },
                                   txtClassEnrollGroupName_id: {
                                           required: true

                                   },
                                   dtExpiryDate:{
                                        
                                        date:true

                                   },
                                   ClassEnrollGroupLimit:{
                                       
                                       min:1
                                   }
                                

                           },
                            errorPlacement: function(error, element) {
                                   element.closest(&quot;.elementclass&quot;).next().html('');
                                  error.appendTo( element.closest(&quot;.elementclass&quot;).next());
                             }

                       });   
                       
                       function resetAddRestrictionsForm(){
                           
                            $('#txtClassEnrollGroupName').val('');
                            $('#ClassEnrollGroupLimit').val('');
                            $('#dtExpiryDate').val('');
                            $(&quot;#action_id&quot;).val('');
                            $(&quot;#restriction_action&quot;).val('');
                           
                       }
                       
                        $('.table_body').on('click', '.btn_delete_group', function(e) { 
                           
                            e.preventDefault();
                            var name=$(this).attr('data-groupname');
                            var id=$(this).attr('data-groupid');
                            var r = confirm(&quot;Are you sure you want to delete &quot;+name+ &quot;?&quot;);
                            if (r == true) {
                                    //waitingDialog.hide();
                                    //waitingDialog.show('Loading....');
                                     $.ajax({
                                         type: &quot;GET&quot;,
                                         cache: false,
                                         timeout:60000,
                                         url: &quot;/PGCTechnologyServices/course-manager/deletegroup?group_id=&quot;+id+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime(),
                                          beforeSend:function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                             waitingDialog.show('Loading....');
                                          }, 
                                         success: function(data) {

                                             data=jQuery.parseJSON(data);
                                             $(&quot;#groups_res_body&quot;).html(data.restrictions_html);
                                             //waitingDialog.hide();
                                             showErrosIfAnySingle();
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide(); 

                                         },
                                          error: function (xhr, ajaxOptions, thrownError) {
                                              
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide(); 
                                             //alert(xhr.status);
                                             //alert(thrownError);
                                             //waitingDialog.hide();
                                           },
                                           complete: function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                         }         
                                                   

                                     });

                            }



                        });
                       $('.table_body').on('click', '.btn_edit_group', function(e) { 
                           
                           e.preventDefault();
                           $(&quot;#btn_add-enrollment-restrictions&quot;).trigger('click');
                           $(&quot;#restriction_action&quot;).val('edit');
                           var name=$(this).attr('data-groupname');
                           var id=$(this).attr('data-groupid');
                           var limit=$(this).attr('data-grouplimit');
                           var remove_after=$(this).attr('data-remove_after');
                           
                           $(&quot;#action_id&quot;).val(id);
                           $(&quot;#txtClassEnrollGroupName&quot;).val(name);
                           $(&quot;#txtClassEnrollGroupName_id&quot;).val(id);
                           $(&quot;#ClassEnrollGroupLimit&quot;).val(limit);
                           $(&quot;#dtExpiryDate&quot;).val(remove_after);
                           $(&quot;#txtClassEnrollGroupName&quot;).attr('disabled','disabled');
                           
                        });
                        
                        $(&quot;#btn_add-enrollment-restrictions&quot;).click(function(e) {
                            
                             $(&quot;#txtClassEnrollGroupName&quot;).removeAttr('disabled');
                             $(&quot;#restriction_action&quot;).val('add');
                            
                             
                        });
                        
                       $(&quot;#txtClassEnrollGroupName1&quot;).change(function(e) { 
                           
                           
                            $.ajax({
                                    type: &quot;GET&quot;,
                                    cache: false,
                                    timeout:60000,
                                    url: &quot;/PGCTechnologyServices/course-manager/checkgroup?groupname=&quot;+ $(&quot;#txtClassEnrollGroupName&quot;).val()+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime(),
                                    beforeSend:function() {
                                             
                                        waitingDialog.hide();
                                        $('#ajax_instr_loader').hide();
                                        waitingDialog.show('Loading....');
                                     }, 
                                    success: function(data) {

                                       data=jQuery.parseJSON(data);
                                       if(data.error==&quot;true&quot;){
                                           
                                            $(&quot;#txtClassEnrollGroupName&quot;).val('');
                                           alert(data.error_msg);
                                           
                                       } 
                                       else{
                                          
                                             $(&quot;#txtClassEnrollGroupName&quot;).val(data.groupname);
                                           
                                       }
                                       
                                        waitingDialog.hide();
                                        $('#ajax_instr_loader').hide();
                                       
                                    },
                                     error: function (xhr, ajaxOptions, thrownError) {
                                         
                                        waitingDialog.hide();
                                        $('#ajax_instr_loader').hide(); 
                                        //alert(xhr.status);
                                        //alert(thrownError);
                                        //waitingDialog.hide();
                                     },
                                      complete: function() {
                                             
                                            waitingDialog.hide();
                                            $('#ajax_instr_loader').hide();
                                       }

                                });
                           
        
                        });
                       $(&quot;#btn_add_restriction&quot;).click(function(e) {
                            e.preventDefault(); 
                            $(&quot;#txtClassEnrollGroupName&quot;).removeAttr('disabled');
                            var myform = $('#frm_restriction_add');
                            var serialized = myform.serialize();

                    
                             if ($(&quot;#frm_restriction_add&quot;).valid()) {

                                    //waitingDialog.hide();
                                     //waitingDialog.show('Loading....');
                                     
                                     var datastring = serialized;
                                      $.ajax({
                                          type: &quot;POST&quot;,
                                          cache: false,
                                          timeout:60000,
                                          url: &quot;/PGCTechnologyServices/course-manager/addgroup?sid=kcbtrojn&amp;time=&quot;+new Date().getTime(),
                                          data: datastring,
                                          beforeSend:function() {
                                             
                                            waitingDialog.hide();
                                            $('#ajax_instr_loader').hide();
                                            waitingDialog.show('Loading....');
                                         }, 
                                          success: function(data) {
                                              
                                              data=jQuery.parseJSON(data);
                                              
                                              if(data.error==&quot;true&quot;){
                                                  
                                                  alert(data.message);
                                              }
                                              else{
                                                  
                                                    $(&quot;#groups_res_body&quot;).html(data.restrictions_html);
                                                    
                                                     resetAddRestrictionsForm();
                                                    var magnificPopup = $.magnificPopup.instance; 
                                                   // save instance in magnificPopup variable
                                                    magnificPopup.close(); 
                                                    //waitingDialog.hide();
                                                    //var obj = jQuery.parseJSON(data); if the dataType is not specified as json uncomment this
                                                    // do what ever you want with the server response
                                               }
                                               showErrosIfAnySingle();
                                               waitingDialog.hide();
                                               $('#ajax_instr_loader').hide();
                                          },
                                           error: function (xhr, ajaxOptions, thrownError) {
                                               
                                              waitingDialog.hide();
                                              $('#ajax_instr_loader').hide(); 
                                              //alert(xhr.status);
                                              //alert(thrownError);
                                              //waitingDialog.hide();
                                            },
                                            complete: function() {
     
                                                waitingDialog.hide();
                                                $('#ajax_instr_loader').hide();
                                            }

                                      });

                                }
                      });
                       
            

        
    
    
            
            
            
                
                    Rubrics
                
                 
                    
                    
                        
                            
                                Rubric*
                                
                                    Choose...  
                                                                         Administrator.. 
                                                                         Auxilary Staff 
                                                                         Knowledge and Skills - pgc 
                                                                         New Sample 
                                                                         Non-Certified staff 
                                                                         pasampless 
                                                                         See I am adding a new item, but it will not show up in tab2. 
                                                                         Teacher 
                                    
                                
                                
                            
                            
                                Domain*
                                
                                    Choose... 
                                
                                
                            
                        

                    
                        
                        Component*
                        
                        
                        Note: Hold down Ctrl key to select multiple.
                    
                        Submit
                    
                
                  
                    


                         $(&quot;#Rubrics&quot;).change(function(e) {


                         })
                        function loadRubricComponents(){

                            var Rubrics=$(&quot;#Rubrics&quot;).val();
                            var domain=$(&quot;#Domain&quot;).val();

                              $.ajax({

                                   type: &quot;GET&quot;,
                                   cache: false,
                                   timeout:60000,
                                   url: &quot;/PGCTechnologyServices/course-manager/filllevel3?rubric=&quot; + Rubrics+'&amp;gradecluster='+domain+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime(),
                                   data: {},
                                   beforeSend:function() {

                                        waitingDialog.hide();
                                        $('#ajax_instr_loader').hide();
                                        waitingDialog.show('Loading....');
                                     }, 
                                    success: function(jsonData) {

                                        $('#rubric_comp').empty().append(jsonData);
                                        
                                         waitingDialog.hide();
                                         $('#ajax_instr_loader').hide();
                                    },
                                    complete: function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                         }
                                })

                        }


                          $.validator.setDefaults({ ignore: '' });

                            $('#frm_rubric_add').validate({

                                     rules: {
                                             Rubrics: {
                                                     required: true

                                             },
                                             Domain: {
                                                     required: true

                                             },
                                             &quot;rubric_comp[]&quot;:{
                                                  required: true,

                                             }
                                     },
                                      errorPlacement: function(error, element) {
                                            error.appendTo( element.next());
                                       }

                                 });  


                               function resetAddRubricForm(){


                                   $(&quot;#rubric_comp option:selected&quot;).prop(&quot;selected&quot;, false);
                                   $(&quot;#Rubrics&quot;).val('');
                                   $(&quot;#Domain&quot;).val('');
                                   $(&quot;#Rubrics&quot;).trigger('change');

                               }

                               $(&quot;#btn_rubric_submit&quot;).click(function(e) {

                                    e.preventDefault(); 
                                    var myform = $('#frm_rubric_add');
                                    var serialized = myform.serialize();


                                     if ($(&quot;#frm_rubric_add&quot;).valid()) {

                                            //waitingDialog.hide();
                                             //waitingDialog.show('Loading....');

                                             var datastring = serialized;
                                              $.ajax({
                                                  type: &quot;POST&quot;,
                                                  cache: false,
                                                  timeout:60000,
                                                  url: &quot;/PGCTechnologyServices/course-manager/addrubric?sid=kcbtrojn&amp;time=&quot;+new Date().getTime(),
                                                  data: datastring,
                                                  beforeSend:function() {
                                                    waitingDialog.hide();
                                                    $('#ajax_instr_loader').hide();
                                                    waitingDialog.show('Loading....');
                                                 }, 

                                                  success: function(data) {

                                                      data=jQuery.parseJSON(data);

                                                      if(data.error==&quot;true&quot;){

                                                          alert(data.message);
                                                      }
                                                      else{
                                                            $(&quot;#rubric_body&quot;).html(data.rubric_tab_html);
                                                       }

                                                      resetAddRubricForm();
                                                      var magnificPopup = $.magnificPopup.instance; 
                                                     // save instance in magnificPopup variable
                                                      magnificPopup.close(); 
                                                      //waitingDialog.hide();
                                                      //var obj = jQuery.parseJSON(data); if the dataType is not specified as json uncomment this
                                                      // do what ever you want with the server response
                                                     showErrosIfAnySingle();  
                                                     waitingDialog.hide();
                                                     $('#ajax_instr_loader').hide(); 
                                                  },
                                                   error: function (xhr, ajaxOptions, thrownError) {
                                                       
                                                      waitingDialog.hide();
                                                      $('#ajax_instr_loader').hide(); 
                                                      //alert(xhr.status);
                                                      //alert(thrownError);
                                                      //waitingDialog.hide();
                                                    },
                                                    complete: function() {
                                                        waitingDialog.hide();
                                                        $('#ajax_instr_loader').hide();
                                                    }        

                                              });

                                        }
                              });    
                     
                    
                         $(&quot;#Rubrics&quot;).change(function(e) {

                            $.ajax({

                               type: &quot;GET&quot;,
                               cache: false,
                               timeout:60000,
                               url: &quot;/PGCTechnologyServices/course-manager/fillgradecluster?rubric=&quot; + $(&quot;#Rubrics&quot;).val()+'&amp;selected='+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime(),
                               data: {},
                               beforeSend:function() {
                                    waitingDialog.hide();
                                    $('#ajax_instr_loader').hide();
                                    waitingDialog.show('Loading....');
                                 }, 
                                success: function(jsonData) {

                                     $('#Domain').empty().append(jsonData);
                                     loadRubricComponents();
                                     
                                     waitingDialog.hide();
                                     $('#ajax_instr_loader').hide();
                                },
                                complete: function() {

                                    
                                     waitingDialog.hide();
                                     $('#ajax_instr_loader').hide();
                                 }
                            })
                       })  

                        $(&quot;#Domain&quot;).change(function(e) {
                             loadRubricComponents();
                        }) 

                         $('.table_body').on('click', '.delete_rubric', function(e) { 

                            e.preventDefault();
                            var id=$(this).attr('data-id');
                            var r = confirm(&quot;Are you sure you want to delete ?&quot;);
                            if (r == true) {
                                    //waitingDialog.hide();
                                    //waitingDialog.show('Loading....');
                                     $.ajax({
                                         type: &quot;GET&quot;,
                                         cache: false,
                                         timeout:60000,
                                         url: &quot;/PGCTechnologyServices/course-manager/deleterubric?id=&quot;+id+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime(),
                                         beforeSend:function() {
                                             
                                            waitingDialog.hide();
                                            $('#ajax_instr_loader').hide();
                                            waitingDialog.show('Loading....');
                                         }, 
                                         success: function(data) {

                                             data=jQuery.parseJSON(data);
                                             $(&quot;#rubric_body&quot;).html(data.rubric_tab_html);
                                             //waitingDialog.hide();
                                             showErrosIfAnySingle();
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();

                                         },
                                          error: function (xhr, ajaxOptions, thrownError) {
                                              
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide(); 
                                             //alert(xhr.status);
                                             //alert(thrownError);
                                             //waitingDialog.hide();
                                           },
                                           complete: function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                             
                                          }         

                                     });

                            }



                        }); 
                    
            
        
        
         
    
        
            
                Manage Question
            
             
              
                
                    Question Type* 
                    
                        
                            Choose..
                            Multichoice (select one)
                            Multichoice (select many)
                            True/False
                            Yes/No
                            Scale
                            Written Response
                            Instructions
                        
                          
                        
                            
                            
                                Include &quot;Other&quot;
                            
                        
                        
                            
                                
                                    
                                        From    To 
                                        
                                                                                
                                    
                            
                        
                         
                    
                    
                
                
                
                    Answer Length
                    
                        
                            Short
                            Medium
                            Long
                        
                          
                        
                         
                           
                                
                                    
                                           Maximum Selections 
                                            
                                    
                                
                               
                             
                                           
                    
                    
                
                
                 
                    Question*
                    
                        
                            
                        
                    
                    
                
                
                
                    Manage Options
                    
                        
                        
                            
                                 Add Option
                            
                            
                              
                            
                            
                                Option Text*
                                
                                
                                
                                
                                    Cancel
                                    Add
                                    
                                        
                                        var oldval=&quot;&quot;;
                                        var oldele=&quot;&quot;;
                                        var removeValue = function(list, value, separator) {
                                            separator = separator || &quot;,&quot;;
                                            var values = list.split(separator);
                                            for(var i = 0 ; i &lt; values.length ; i++) {
                                              if(values[i] == value) {
                                                values.splice(i, 1);
                                                return values.join(separator);
                                              }
                                            }
                                            return list;
                                          }
                                        $('#opt_grid').on('click', '.QDelete', function(e) { 
                                            
                                              var r = confirm(&quot;Are you sure you want to delete this option?&quot;);
                                              if (r == true) {
                                                    
                                                     var val=$(this).attr(&quot;data-val&quot;);
                                                     var list=$(&quot;#ans&quot;).val();
                                                     $return=removeValue(list,val,&quot;|&quot;);
                                                     $(&quot;#ans&quot;).val($return);
                                                     $(this).parent().remove();
                                       
                                                }    
                                        })
                                        $('#opt_grid').on('click', '.Qedit', function(e) { 
                                            
                                             oldval=$(this).attr(&quot;data-val&quot;);
                                             oldele=this;
                                             $(&quot;#btn_add_opt&quot;).trigger(&quot;click&quot;);
                                             
                                             if($(&quot;#answer_length&quot;).val()==&quot;Short&quot;){ 
                                               
                                                  $(&quot;#opt_short&quot;).val(oldval);
                                                }
                                                else if($(&quot;#answer_length&quot;).val()==&quot;Medium&quot;){ 

                                                    $(&quot;#opt_medium&quot;).val(oldval);
                                                    
                                                }
                                                else if($(&quot;#answer_length&quot;).val()==&quot;Long&quot;){ 

                                                    $(&quot;#opt_long&quot;).val(oldval);
                                                }
                                             
                                                    
                                        })
                                        
                                        $( &quot;#btn_add_opt_add_opt&quot; ).click(function() {
                                         
                                         
                                         
                                          var list=$(&quot;#ans&quot;).val();
                                          $return=removeValue(list,oldval,&quot;|&quot;);
                                          $(&quot;#ans&quot;).val($return);
                                          
                                          if(oldele!=&quot;&quot;){
                                              
                                              $(oldele).parent().remove();
                                              oldele=&quot;&quot;;
                                          }
                                                     
                                           var opt='';
                                           if($(&quot;#answer_length&quot;).val()==&quot;Short&quot;){ 
                                                $(&quot;#ans&quot;).val($(&quot;#ans&quot;).val()+&quot;|&quot;+$(&quot;#opt_short&quot;).val());
                                                opt=$(&quot;#opt_short&quot;).val();
                                            }
                                            else if($(&quot;#answer_length&quot;).val()==&quot;Medium&quot;){ 
                                            
                                                $(&quot;#ans&quot;).val($(&quot;#ans&quot;).val()+&quot;|&quot;+$(&quot;#opt_medium&quot;).val());
                                                opt=$(&quot;#opt_medium&quot;).val();
                                            }
                                            else if($(&quot;#answer_length&quot;).val()==&quot;Long&quot;){ 
                                            
                                                $(&quot;#ans&quot;).val($(&quot;#ans&quot;).val()+&quot;|&quot;+$(&quot;#opt_long&quot;).val());
                                                opt=$(&quot;#opt_long&quot;).val();
                                            }
                                            
                                           if($.trim(opt)==&quot;&quot;){
                                              
                                              alert(&quot;Please enter option value.&quot;)
                                               
                                           } 
                                           else { 
                                               
                                                removeValue($(&quot;#ans&quot;).val(),opt,&quot;|&quot;);
                                                $opt='&lt;li class=&quot;list-group-item&quot;>'+opt+'&lt;button type=&quot;button&quot; class=&quot;btn btn-sm btn-danger f-right QDelete&quot; title=&quot;Delete&quot; data-val=&quot;'+opt+'&quot; >&lt;span class=&quot;fa fa-times&quot;>&lt;/span>&lt;/button>&lt;button type=&quot;button&quot; class=&quot;btn btn-sm btn-info f-right Qedit&quot; style=&quot;margin-right:5px;&quot; title=&quot;Edit&quot; data-val=&quot;'+opt+'&quot;>&lt;span class=&quot;fa fa-pencil&quot;>&lt;/span>&lt;/button>&lt;/li>';
                                                $(&quot;#opt_grid&quot;).append($opt);
                                                $(&quot;#opt_grid&quot;).show();
                                                $('#opt_short').val('');
                                                $('#opt_medium').val('');
                                                $('#opt_long').val('');
                                                $(&quot;#btn_cancel_opt_1&quot;).trigger('click');
                                            
                                            }
                                            
                                             oldval=&quot;&quot;;
                                             oldele=&quot;&quot;;
                                            
                                        });
                                        
                                    
                                
                            
                        
                        
                        
                             
                                 True
                                 False
                             
                        
                        
                        
                           
                             Yes
                             No
                         
                        
                    
                    
                          $('#btnYes_true').click(function () {
                            $('#btnNo_false').addClass('btn-default');
                            $('#btnNo_false').removeClass('btn-danger');
                            $('#btnYes_true').removeClass('btn-default');
                            $('#btnYes_true').addClass('btn-success');
                      
                        });
                        $('#btnNo_false').click(function () {
                            $('#btnNo_false').addClass('btn-danger');
                            $('#btnNo_false').removeClass('btn-default');
                            $('#btnYes_true').addClass('btn-default');
                            $('#btnYes_true').removeClass('btn-success');
                            
                        });
                          $('#btnYes_Q').click(function () {
                            $('#btnNo_Q').addClass('btn-default');
                            $('#btnNo_Q').removeClass('btn-danger');
                            $('#btnYes_Q').removeClass('btn-default');
                            $('#btnYes_Q').addClass('btn-success');
                      
                        });
                        $('#btnNo_Q').click(function () {
                            $('#btnNo_Q').addClass('btn-danger');
                            $('#btnNo_Q').removeClass('btn-default');
                            $('#btnYes_Q').addClass('btn-default');
                            $('#btnYes_Q').removeClass('btn-success');
                            
                        });
                    
                     
                
                
                
                    
                    
                    
                    Submit
                
            
                
                       $.validator.addMethod(&quot;checkQuestionContent&quot;, function(value, element) {
                                    
                                
                            var editorContent = tinyMCE.get('txtQuestion').getContent();
                            if (editorContent == '')
                            {
                                return false;
                            }
                            else
                            {
                                return true;
                            }

                       }, &quot;Please enter question&quot;);
                       
                       $.validator.addMethod('lessThanEqual', function(value, element, param) {
                        var i = parseInt(value);
                        var j = parseInt($(param).val());
                        
                        return i >= j;
                    }, &quot;Scale from cannot be greater than To &quot;);
                    
                       $.validator.addMethod('greaterThan0', function(value, element, param) {
                        var i = parseInt(value);
                        return i >0;
                    }, &quot;Maximum Selections cannot less than 0 &quot;);

                       $.validator.addMethod(&quot;checkAns&quot;, function(value, element) {
                            
                            if($(&quot;#question_type&quot;).val()==&quot;MultichoiceOne&quot; || $(&quot;#question_type&quot;).val()==&quot;MultichoiceMany&quot;){
                               
                               if($(&quot;#ans&quot;).val()==&quot;&quot;){
                                   
                                   return false;
                               }
                               else{
                                   
                                   return true;
                               }
                                
                            }
                            else{
                                
                                return true;
                            }
                               

                       }, &quot;Please add options&quot;);
                       
                     $('#frm_question').validate({

                        rules: {
                                question_type: {
                                        required: true

                                },
                                answer_length: {
                                        required: true

                                },
                                check_Question_desc: {
                                        checkQuestionContent: true

                                },
                                ans:{
                                    
                                    checkAns:true
                                },
                                scale_to: { lessThanEqual: &quot;#scale_from&quot; },
                                maximum_selections: { greaterThan0: true }

                        },
                         errorPlacement: function(error, element) {
                                error.appendTo( element.parent().next());
                          }

                    });
                    
                    
                    function isInt(value) {
                        return !isNaN(value) &amp;&amp; (function(x) { return (x | 0) === x; })(parseFloat(value))
                      }
                     $('.table_body').on('click', '.edit_question', function(e) {
                            
                            $(&quot;#add-manage-question-Questions&quot;).trigger('click');
                            e.preventDefault();
                            var id=$(this).attr('data-id');
                          
                             
                            if($(e.target).hasClass('followup_que')){
                                
                                
                              $(&quot;#followup_or_survey&quot;).val('followup') ; 
                            }
                            else{
                                
                                 $(&quot;#followup_or_survey&quot;).val('survey');  
                                
                            }
                            
                            if($(&quot;#followup_or_survey&quot;).val()==&quot;survey&quot;){
                             
                               var url_=&quot;/PGCTechnologyServices/course-manager/getquestioninfo?id=&quot;+id+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime();
                                 
                             }
                             else{
                                 
                                  var url_=&quot;/PGCTechnologyServices/course-manager/getquestioninfofollowup?id=&quot;+id+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime();
                             }
                            
                            $(&quot;#question_action&quot;).val('edit');
                            $(&quot;#question_action_id&quot;).val(id);
                            
                            
                            
                            $.ajax({
                                 type: &quot;GET&quot;,
                                 cache: false,
                                 timeout:60000,
                                 url: url_,
                                 datatype:'json',
                                 beforeSend:function() {

                                    waitingDialog.hide();
                                    $('#ajax_instr_loader').hide();
                                    waitingDialog.show('Loading....');
                                 }, 
                                 success: function(data) {

                                        data=jQuery.parseJSON(data);
                                        $('#question_type').prop('selectedIndex', data.Qtype);
                                        tinymce.activeEditor.setContent(data.Question);
                                        $('#question_type').trigger(&quot;change&quot;);
                                        
                                        $ans=data.Answers;
                                        $nwAns='';
                                        if(data.Qtype==1 || data.Qtype==2){
                                            
                                            explodeAns=$ans.split(&quot;|&quot;);
                                             for(var i = 0 ; i &lt; explodeAns.length ; i++) {
                                              
                                                $val_ans=explodeAns[i];
                                                if($.trim($val_ans)!=''){
                                                    $opt='&lt;li class=&quot;list-group-item&quot;>'+$val_ans+'&lt;button type=&quot;button&quot; class=&quot;btn btn-sm btn-danger f-right QDelete&quot; title=&quot;Delete&quot; data-val=&quot;'+$val_ans+'&quot; >&lt;span class=&quot;fa fa-times&quot;>&lt;/span>&lt;/button>&lt;button type=&quot;button&quot; class=&quot;btn btn-sm btn-info f-right Qedit&quot; style=&quot;margin-right:5px;&quot; title=&quot;Edit&quot; data-val=&quot;'+$val_ans+'&quot;>&lt;span class=&quot;fa fa-pencil&quot;>&lt;/span>&lt;/button>&lt;/li>';
                                                    $(&quot;#opt_grid&quot;).append($opt);
                                                    $nwAns+=$val_ans+'|';
                                                }   
                                             
                                            }
                                            
                                            if($nwAns.slice(-1)=='|'){
                                            
                                               $nwAns = $nwAns.slice(0, -1);
                                            }
                                            $('#ans').val($nwAns);
                                            
                                        }
                                        else if(data.Qtype==5){
                                        
                                            var from=0;
                                            var to=0;
                                            var flag=false;
                                            
                                            explodeAns=$ans.split(&quot;|&quot;);
                                             for(var i = 0 ; i &lt; explodeAns.length ; i++) {
                                               
                                               if(isInt(explodeAns[i])){
                                                  
                                                  if(flag==false){
                                                      
                                                    $(&quot;#scale_from&quot;).val(parseInt(explodeAns[i])); 
                                                    flag=true;
                                                  }
                                                  else{
                                                      
                                                        $(&quot;#scale_to&quot;).val(parseInt(explodeAns[i]));
                                                   }
                                                   
                                                }
                                             }
                                        }
                                        $(&quot;#maximum_selections_val&quot;).val(data.MaxAnswer);
                                        if(data.Other==&quot;1&quot;){
                                        
                                            $( &quot;#checkbox2&quot; ).prop( &quot;checked&quot;, true );
                                        
                                        }
                                        else{
                                        
                                             $( &quot;#checkbox2&quot; ).prop( &quot;checked&quot;, false );
                                        }
                                       
                                       if(data.AnswerLength==1){
                                       
                                         $('#answer_length option:eq(0)').prop('selected', true);
                                       }
                                       else if(data.AnswerLength==2){
                                       
                                         $('#answer_length option:eq(1)').prop('selected', true);
                                       }
                                       else if(data.AnswerLength==3){
                                       
                                         $('#answer_length option:eq(2)').prop('selected', true);
                                       }
                                    
                                     waitingDialog.hide();
                                     $('#ajax_instr_loader').hide();

                                 },
                                  error: function (xhr, ajaxOptions, thrownError) {
                                  
                                    waitingDialog.hide();
                                    $('#ajax_instr_loader').hide();
                                     //alert(xhr.status);
                                     //alert(thrownError);
                                     //waitingDialog.hide();
                                   },
                                  complete: function() {

                                        waitingDialog.hide();
                                        $('#ajax_instr_loader').hide();
                                    }  

                             });


                    })
                            
                     $('.table_body').on('click', '.delete_question', function(e) { 
                           
                           
                            if($(e.target).hasClass('followup_que')){
                                
                              $(&quot;#followup_or_survey&quot;).val('followup') ; 
                            }
                            else{
                                
                                 $(&quot;#followup_or_survey&quot;).val('survey');  
                                
                             }
                            
                             e.preventDefault();
                             var id=$(this).attr('data-id');
                             
                             
                             if($(&quot;#followup_or_survey&quot;).val()==&quot;survey&quot;){
                           
                               var url_=&quot;/PGCTechnologyServices/course-manager/deletequestion?id=&quot;+id+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime()
                                 
                             }
                             else{
                                 
                                  var url_=&quot;/PGCTechnologyServices/course-manager/deletequestionfollowup?id=&quot;+id+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime()
                             }
                             
                             var r = confirm(&quot;Are you sure you want to delete this question?&quot;);
                             if (r == true) {

                                    //waitingDialog.hide();
                                    //waitingDialog.show('Loading....');
                                     $.ajax({
                                         type: &quot;GET&quot;,
                                         cache: false,
                                         timeout:60000,
                                         url: url_,
                                          beforeSend:function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                             waitingDialog.show('Loading....');
                                          }, 
                                         success: function(data) {

                                            if($(&quot;#followup_or_survey&quot;).val()==&quot;survey&quot;){
                                                
                                                $(&quot;#Survery_Question_Body&quot;).html(data);
                                            }
                                            else{
                                                
                                                $(&quot;#Followup_Question_Body&quot;).html(data);
                                            }
                                            showErrosIfAnySingle();
                                            waitingDialog.hide();
                                            $('#ajax_instr_loader').hide();
                                         },
                                          error: function (xhr, ajaxOptions, thrownError) {
                                              
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide(); 
                                             //alert(xhr.status);
                                             //alert(thrownError);
                                             //waitingDialog.hide();
                                           },
                                           complete: function() {
                                             
                                                 waitingDialog.hide();
                                                $('#ajax_instr_loader').hide();
                                                
                                            } 

                                     });

                            }



                        });
                        
                    
                    function resetAddQuestionForm(){
                        
                        $(&quot;#question_type option:eq(0)&quot;).prop('selected', true);
                        $(&quot;#answer_length option:eq(0)&quot;).prop('selected', true);
                        $(&quot;#scale_from&quot;).val(0);
                        $(&quot;#scale_to&quot;).val(0);
                        $(&quot;#maximum_selections_val&quot;).val(1);
                        $(&quot;#ans&quot;).val(&quot;&quot;);
                        $(&quot;#opt_grid&quot;).html('');
                        $(&quot;#question_type&quot;).trigger(&quot;change&quot;);
                        $(&quot;#txtQuestion&quot;).val(&quot;&quot;);
                        
                    }
                     $( &quot;#btn_submit_question&quot; ).click(function(e) {
                         
                         
                           $(&quot;#txtQuestion&quot;).val(tinymce.get('txtQuestion').getContent());
                          
                           var myform = $('#frm_question');

                            // Find disabled inputs, and remove the &quot;disabled&quot; attribute
                           var disabled = myform.find(':input:disabled').removeAttr('disabled');

                            $('#btn_submit_question').validate();

                            // serialize the form
                           var serialized = myform.serialize();
                           
                           //alert(tinymce.get('txtQuestion').getContent());
                           //alert(serialized);
                           //     serialized[item].value =  tinymce.get('txtQuestion').getContent();
                           
                       

                            // re-disabled the set of inputs that you previously enabled
                           disabled.attr('disabled','disabled');

                           e.preventDefault(); 

                           if($(&quot;#followup_or_survey&quot;).val()==&quot;survey&quot;){
                             
                               var url_=&quot;/PGCTechnologyServices/course-manager/addquestion?sid=kcbtrojn&amp;time=&quot;+new Date().getTime();
                                 
                             }
                            else{
                                
                                
                                var url_=&quot;/PGCTechnologyServices/course-manager/addquestionfollowup?sid=kcbtrojn&amp;time=&quot;+new Date().getTime();
                            } 
                            if ($(&quot;#frm_question&quot;).valid()) {

                                    //waitingDialog.hide();
                                    //waitingDialog.show('Loading....');
                                    var datastring = serialized;
                                     $.ajax({
                                         type: &quot;POST&quot;,
                                         cache: false,
                                         timeout:60000,
                                         url: url_,
                                         data: datastring,
                                         beforeSend:function() {
                                             
                                            waitingDialog.hide();
                                            $('#ajax_instr_loader').hide();
                                            waitingDialog.show('Loading....');
                                         }, 
                                         success: function(data) {
                                             
                                              if($(&quot;#followup_or_survey&quot;).val()==&quot;survey&quot;){
                                                  
                                                  $(&quot;#Survery_Question_Body&quot;).html(data);
                                              }
                                              else{
                                                  
                                                  $(&quot;#Followup_Question_Body&quot;).html(data);
                                              }
                                             //$(&quot;#instructor_tbody&quot;).html(data);
                                             resetAddQuestionForm();
                                             var magnificPopup = $.magnificPopup.instance; 
                                            // save instance in magnificPopup variable
                                             magnificPopup.close(); 
                                             //waitingDialog.hide();
                                             //var obj = jQuery.parseJSON(data); if the dataType is not specified as json uncomment this
                                             // do what ever you want with the server response
                                             showErrosIfAnySingle();
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                             
                                         },
                                          error: function (xhr, ajaxOptions, thrownError) {
                                              
                                            waitingDialog.hide();
                                             $('#ajax_instr_loader').hide(); 
                                             //alert(xhr.status);
                                             //alert(thrownError);
                                             //waitingDialog.hide();
                                           },
                                           complete: function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                         } 

                                     });

                               }
                         
                         
                     })
                
                
        
    
    
    
    
        
    
    
        
          
            
                Add Instructor
            
            
                
                    Name*
                    
                       
                        
                        
                        
                        
                    
                    
                    
                
                
                    Telephone
                    
                        
                       
                    
                     
                
                
                
                    Email*
                    
                        
                      
                    
                      
                
                
                
                    Upload Picture
                    
                        
                            
                        
                         
                            function readURL(input) {
                                    if (input.files &amp;&amp; input.files[0]) {

                                         var fileToLoad = input.files[0];
                                         var filetype = fileToLoad.type.toLowerCase();

                                         if (filetype == 'image/jpeg' || filetype == 'image/png') {

                                            var reader = new FileReader();
                                            reader.onload = function (e) {
                                                    $('#ins_profile_pic').attr('src', e.target.result);
                                                    $('#test_insp_base64').val(e.target.result);
                                            };
                                            reader.readAsDataURL(input.files[0]);

                                        }
                                        else{

                                            alert(&quot;Please only upload png or jpeg image only&quot;);
                                            return false;

                                        }
                                    }
                            }
                        
                        
                            
                                
                                    BROWSE
                                    
                                    
                                
                            
                        
                    
                   
                
                
                    Reset 
                    Submit
                
            
        
        
         
                         
              $('#frm_submit_instructor').validate({

                    rules: {
                            txtInstructor: {
                                    required: true,
                                   
                            },
                            ins_email: {
                                    required: true,
                                    email: true
                            },

                    },
                     errorPlacement: function(error, element) {
                         
                             error.appendTo( $(element).closest('.elementclass').next());
                             
                            
                      }

                });           
              
            var orginal_src=$(&quot;#ins_profile_pic&quot;).attr('src');  
            $('#txtInstructor').bootcomplete({
                url:'/PGCTechnologyServices/course-manager/getinstructor',
                'idField' :true,
                'minLength':7,
                callback: function() {
                  
                  InstructorChange();
                 
                }
                
            });
            
             $(&quot;#txtInstructor&quot;).change(function(e) {

                InstructorChange();
             });
            
            
            $('.table_body').on('click', '.delete_instructor', function(e) { 
                console.log(this);
                e.preventDefault();
                var name=$(this).attr('data-name');
                var id=$(this).attr('data-id');
                var r = confirm(&quot;Are you sure you want to delete &quot;+name+ &quot;?&quot;);
                if (r == true) {
                    
                        //waitingDialog.hide();
                        //waitingDialog.show('Loading....');
                         $.ajax({
                             type: &quot;GET&quot;,
                             cache: false,
                             timeout:60000,
                             url: &quot;/PGCTechnologyServices/course-manager/deleteinstructor?instructor_id=&quot;+id+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime(),
                              beforeSend:function() {
                                             
                                waitingDialog.hide();
                                $('#ajax_instr_loader').hide();
                                waitingDialog.show('Loading....');
                             }, 
                             success: function(data) {

                                 $(&quot;#instructor_tbody&quot;).html(data);
                                 //waitingDialog.hide();
                                 showErrosIfAnySingle();
                                 waitingDialog.hide();
                                 $('#ajax_instr_loader').hide();
                                 
                             },
                              error: function (xhr, ajaxOptions, thrownError) {
                                  
                                 waitingDialog.hide();
                                 $('#ajax_instr_loader').hide(); 
                                 //alert(xhr.status);
                                 //alert(thrownError);
                                 //waitingDialog.hide();
                               },
                               complete: function() {
                                             
                                    waitingDialog.hide();
                                    $('#ajax_instr_loader').hide();
                                }         

                         });
                    
                }
                
                 
                 
            });
            
          
            
            
            $(&quot;#ins_email&quot;).change(function(e) {
            
                e.preventDefault();
                if($('[name=&quot;txtInstructor_id&quot;]').val()==''){
                    
                    
                      $.ajax({
                             type: &quot;GET&quot;,
                             cache: false,
                             timeout:60000,
                             url: &quot;/PGCTechnologyServices/course-manager/getinstructordatabyemail?email=&quot;+$('#ins_email').val()+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime(),
                              beforeSend:function() {
                                             
                                waitingDialog.hide();
                                $('#ajax_instr_loader').hide();
                                waitingDialog.show('Loading....');
                             }, 
                             success: function(data) {

                                    data=jQuery.parseJSON(data);   
                                    if($.trim(data.ClientID)!=''){

                                        $('[name=&quot;txtInstructor_id&quot;]').val(data.ClientID);
                                        $('[name=&quot;txtInstructor__id&quot;]').val(data.ID);
                                        $('[name=&quot;txtInstructor__userid&quot;]').val(data.UserID);
                                        $(&quot;#txtInstructor&quot;).val(data.UserName);
                                        $(&quot;#txtInstructor&quot;).attr('disabled','disabled');
                                        $(&quot;#ins_telephone&quot;).val(data.phone1);
                                        $(&quot;#ins_telephone&quot;).attr('disabled','disabled');
                                        $(&quot;#ins_telephone&quot;).addClass('course_wizard_disabled_field');
                                        $(&quot;#ins_email&quot;).val(data.Email);
                                        $(&quot;#ins_email&quot;).attr('disabled','disabled');
                                        $(&quot;#ins_email&quot;).addClass('course_wizard_disabled_field');
                                        $(&quot;#ins_profile_pic&quot;).attr('src',data.Profile_Pic);
                                        $(&quot;.browse_pic #test_insp&quot;).attr('disabled','disabled');
                                        $(&quot;.browse_pic #test_insp&quot;).addClass('course_wizard_disabled_field');
                                        $(&quot;.browse_pic #browse_insp_img&quot;).attr('disabled','disabled');
                                        $(&quot;.browse_pic #browse_insp_img&quot;).addClass('course_wizard_disabled_field');

                                    }
                                    
                                  waitingDialog.hide();
                                  $('#ajax_instr_loader').hide();    
                                 
                             },
                              error: function (xhr, ajaxOptions, thrownError) {
                                  
                                 waitingDialog.hide();
                                 $('#ajax_instr_loader').hide(); 
                                 //alert(xhr.status);
                                 //alert(thrownError);
                                 //waitingDialog.hide();
                               },
                               complete: function() {
                                             
                                    waitingDialog.hide();
                                    $('#ajax_instr_loader').hide();
                                }         

                         });
                    
                  
                    
                    
                }
            
            });
            
            function InstructorChange(){

                //waitingDialog.hide();  
                //waitingDialog.show('Loading....');

                        setTimeout(function(){
                        if($('[name=&quot;txtInstructor_id&quot;]').val()!=''){


                                 $.ajax({
                                        type: &quot;GET&quot;,
                                        cache: false,
                                        timeout:60000,
                                        url: &quot;/PGCTechnologyServices/course-manager/getinstructordata?client_id=&quot;+$('[name=&quot;txtInstructor_id&quot;]').val()+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime(),
                                         beforeSend:function() {

                                           waitingDialog.hide();
                                           $('#ajax_instr_loader').hide();
                                           waitingDialog.show('Loading....');
                                        }, 
                                        success: function(data) {

                                                data=jQuery.parseJSON(data);   
                                                $(&quot;#txtInstructor__id&quot;).val(data.ID);
                                                $(&quot;#txtInstructor__userid&quot;).val(data.UserID);
                                                $(&quot;#ins_telephone&quot;).val(data.phone1);
                                                $(&quot;#ins_telephone&quot;).attr('disabled','disabled');
                                                $(&quot;#ins_telephone&quot;).addClass('course_wizard_disabled_field');
                                                $(&quot;#ins_email&quot;).val(data.Email);
                                                $(&quot;#ins_email&quot;).attr('disabled','disabled');
                                                $(&quot;#ins_email&quot;).addClass('course_wizard_disabled_field');
                                                $(&quot;#ins_profile_pic&quot;).attr('src',data.Profile_Pic);
                                                $(&quot;.browse_pic #test_insp&quot;).attr('disabled','disabled');
                                                $(&quot;.browse_pic #test_insp&quot;).addClass('course_wizard_disabled_field');
                                                $(&quot;.browse_pic #browse_insp_img&quot;).attr('disabled','disabled');
                                                $(&quot;.browse_pic #browse_insp_img&quot;).addClass('course_wizard_disabled_field');
                                                
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide(); 

                                        },
                                         error: function (xhr, ajaxOptions, thrownError) {

                                            waitingDialog.hide();
                                            $('#ajax_instr_loader').hide(); 
                                            //alert(xhr.status);
                                            //alert(thrownError);
                                            //waitingDialog.hide();
                                          },
                                          complete: function() {

                                               waitingDialog.hide();
                                               $('#ajax_instr_loader').hide();
                                           }         

                                    });
                         
                         
                            } 
                           else{
                               
                                
                                $(&quot;#txtInstructor__id&quot;).val('');
                                $(&quot;#txtInstructor__userid&quot;).val('');
                                $(&quot;#txtInstructor_id&quot;).val('');
                                $(&quot;#ins_telephone&quot;).val('');
                                $(&quot;#ins_telephone&quot;).removeAttr('disabled');
                                $(&quot;#ins_telephone&quot;).removeClass('course_wizard_disabled_field');
                                $(&quot;#ins_email&quot;).val('');
                                $(&quot;#ins_email&quot;).removeAttr('disabled');
                                $(&quot;#ins_email&quot;).removeClass('course_wizard_disabled_field');
                                $(&quot;#ins_profile_pic&quot;).attr('src',orginal_src);
                                $(&quot;.browse_pic #test_insp&quot;).removeAttr('disabled');
                                $(&quot;.browse_pic #test_insp&quot;).removeClass('course_wizard_disabled_field');
                                $(&quot;.browse_pic #browse_insp_img&quot;).removeAttr('disabled');
                                $(&quot;.browse_pic #browse_insp_img&quot;).removeClass('course_wizard_disabled_field');
                                waitingDialog.hide();
                                $('#ajax_instr_loader').hide(); 
                               
                               
                           } 
                        }, 1000);

                      //waitingDialog.hide();


            }
            
            
             $(&quot;#reset,#add-instructor_link&quot;).click(function(e) {
                
                $(&quot;.bc-menu&quot;).html('');
                e.preventDefault(); 
                resetAddInstructorForm();
               
             });
             
             $(&quot;#submit_instructor&quot;).click(function(e) {
                 
                 
                   if($('[name=&quot;txtInstructor_id&quot;]').val()==&quot;&quot;){
                      
                       alert(&quot;No matching names were found in the directory. Don't worry we will assume this is an external intructor&quot;);
                       
                   }
                   var myform = $('#frm_submit_instructor');

                    // Find disabled inputs, and remove the &quot;disabled&quot; attribute
                   var disabled = myform.find(':input:disabled').removeAttr('disabled');

                    $('#frm_submit_instructor').validate();
                    
                    // serialize the form
                   var serialized = myform.serialize();
                   
                    // re-disabled the set of inputs that you previously enabled
                   disabled.attr('disabled','disabled');
                   
                   e.preventDefault(); 
                  
                    if ($(&quot;#frm_submit_instructor&quot;).valid()) {
                        
                            //waitingDialog.hide();
                            //waitingDialog.show('Loading....');
                            var datastring = serialized;
                             $.ajax({
                                 type: &quot;POST&quot;,
                                 cache: false,
                                 timeout:60000,
                                 url: &quot;/PGCTechnologyServices/course-manager/addinstructor?sid=kcbtrojn&amp;time=&quot;+new Date().getTime(),
                                 data: datastring,
                                 beforeSend:function() {
                                             
                                    waitingDialog.hide();
                                    $('#ajax_instr_loader').hide();
                                    waitingDialog.show('Loading....');
                                 }, 
                                 success: function(data) {

                                     $(&quot;#instructor_tbody&quot;).html(data);
                                     resetAddInstructorForm();
                                     var magnificPopup = $.magnificPopup.instance; 
                                    // save instance in magnificPopup variable
                                     magnificPopup.close(); 
                                     //waitingDialog.hide();
                                     //var obj = jQuery.parseJSON(data); if the dataType is not specified as json uncomment this
                                     // do what ever you want with the server response
                                     showErrosIfAnySingle();
                                     waitingDialog.hide();
                                     $('#ajax_instr_loader').hide();
                                 },
                                  error: function (xhr, ajaxOptions, thrownError) {
                                      
                                      waitingDialog.hide();
                                     $('#ajax_instr_loader').hide(); 
                                     //alert(xhr.status);
                                     //alert(thrownError);
                                     //waitingDialog.hide();
                                  },
                                    complete: function() {
                                             
                                        waitingDialog.hide();
                                        $('#ajax_instr_loader').hide();
                                    }         

                             });
                
                       }
             });
             
             
             
            function resetAddInstructorForm(){
                
                    
                    $(&quot;#txtInstructor&quot;).removeAttr('disabled');
                    $(&quot;#txtInstructor&quot;).removeClass('course_wizard_disabled_field');
                    $(&quot;#txtInstructor_id&quot;).val('');
                    $(&quot;#txtInstructor__id&quot;).val('');
                    $(&quot;#txtInstructor__userid&quot;).val('');
                    $(&quot;#txtInstructor&quot;).val('');
                    $(&quot;#ins_telephone&quot;).val('');
                    $(&quot;#ins_telephone&quot;).removeAttr('disabled');
                    $(&quot;#ins_telephone&quot;).removeClass('course_wizard_disabled_field');
                    $(&quot;#ins_email&quot;).val('');
                    $(&quot;#ins_email&quot;).removeAttr('disabled');
                    $(&quot;#ins_email&quot;).removeClass('course_wizard_disabled_field');
                    $(&quot;#ins_profile_pic&quot;).attr('src',orginal_src);
                    $(&quot;.browse_pic #test_insp&quot;).removeAttr('disabled');
                    $(&quot;.browse_pic #test_insp&quot;).removeClass('course_wizard_disabled_field');
                    $(&quot;.browse_pic #browse_insp_img&quot;).removeAttr('disabled');
                    $(&quot;.browse_pic #browse_insp_img&quot;).removeClass('course_wizard_disabled_field');
                
            }

         
    
    
    
    
       
           
           
           
         
            
                Manage Accounting Information
            
            
                
                    Training Year*
                    
                        
                            Choose..
                                                            
                              2006-2007
                            
                                                            
                              2007-2008
                            
                                                            
                              2008-2009
                            
                                                            
                              2009-2010
                            
                                                            
                              2010-2011
                            
                                                            
                              2011-2012
                            
                                                            
                              2012-2013
                            
                                                            
                              2013-2014
                            
                                                            
                              2014-2015
                            
                                                            
                              2015-2016
                            
                                                            
                              2016-2017
                            
                                                            
                              2017-2018
                            
                                                            
                              2018-2019
                            
                                                            
                              2019-2020
                            
                                                            
                              2020-2021
                            
                                                            
                              School Month
                            
                                                    
                    
                    
                
                
                    Calculate Based On Enrollment
                    
                     
                      
                            
                            function resetAcooutingFields(){
                                
                                var original_trainingYear='1035';
                                
                                $('#cboFiscalYear option[value=&quot;'+original_trainingYear+'&quot;]').prop(&quot;selected&quot;, true);
                                $(&quot;#btnNo&quot;).trigger('click');
                                $(&quot;#cboPerSeat&quot;).val(0);
                                $(&quot;#action_id&quot;).val('');
                                $(&quot;#txtAmount&quot;).val('');
                                
                                             
                                
                            }
                            
                            
                            
                            $(&quot;#add-accounting-information-btn&quot;).click(function(e){
                               
                               e.preventDefault();
                               resetAcooutingFields();
                               $(&quot;#action&quot;).val('add');  
                            });
                            
                            $('.table_body').on('click', '.edit_accounting', function(e) {
                                
                                    $(&quot;#add-accounting-information-btn&quot;).trigger('click');
                                    e.preventDefault();
                                    var id=$(this).attr('id');
                                    var explode_arr=id.split(&quot;_&quot;);
                                    var exid=explode_arr[2];
                                    //waitingDialog.hide();
                                    //waitingDialog.show('Loading....');
                                    $(&quot;#action&quot;).val('edit');
                                    $(&quot;#action_id&quot;).val(exid);
                                    
                                    $.ajax({
                                         type: &quot;GET&quot;,
                                         cache: false,
                                         timeout:60000,
                                         url: &quot;/PGCTechnologyServices/course-manager/getsingleaccouting?id=&quot;+exid+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime(),
                                         datatype:'json',
                                          beforeSend:function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                             waitingDialog.show('Loading....');
                                          }, 
                                         success: function(data) {
                                             
                                             data=jQuery.parseJSON(data)

                                             
                                             $(&quot;#cboFiscalYear&quot;).val(data.FiscalYear);
                                             $(&quot;#cboFiscalYear_text&quot;).val(data.FiscalYear_text);
                                             $(&quot;#cboPerSeat&quot;).val(data.PerSeat);
                                             $(&quot;#txtAmount&quot;).val(data.Amount);
                                             
                                             if(parseInt(data.PerSeat)==1){
                                                 
                                                $(&quot;#btnYes&quot;).trigger('click'); 
                                             }
                                             else{
                                                 
                                                 $(&quot;#btnNo&quot;).trigger('click');
                                             }
                                             
                                                                                          //waitingDialog.hide();
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide(); 

                                         },
                                          error: function (xhr, ajaxOptions, thrownError) {
                                              
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide(); 
                                             //alert(xhr.status);
                                             //alert(thrownError);
                                             //waitingDialog.hide();
                                           },
                                           complete: function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                         }         

                                     });
                                    

                            })
                            
                            $('.table_body').on('click', '.delete_accounting', function(e) { 
                                
                            e.preventDefault();
                            
                            var id=$(this).attr('id');
                            var explode_arr=id.split(&quot;_&quot;);
                            var exid=explode_arr[2];
                            
                            var r = confirm(&quot;Are you sure you want to delete ?&quot;);
                            if (r == true) {
                                    //waitingDialog.hide();
                                    //waitingDialog.show('Loading....');
                                     $.ajax({
                                         type: &quot;GET&quot;,
                                         cache: false,
                                         timeout:60000,
                                         url: &quot;/PGCTechnologyServices/course-manager/deleteaccouting?id=&quot;+exid+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime(),
                                          beforeSend:function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                             waitingDialog.show('Loading....');
                                          }, 
                                         success: function(data) {

                                             $(&quot;#accouting_table&quot;).html(data);
                                             //waitingDialog.hide();
                                             showErrosIfAnySingle();
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();

                                         },
                                          error: function (xhr, ajaxOptions, thrownError) {
                                              
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide(); 
                                             //alert(xhr.status);
                                             //alert(thrownError);
                                             //waitingDialog.hide();
                                           },
                                          complete: function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                         }          

                                     });

                            }



                        });
            
                            $(&quot;#add-accounting-information-btn&quot;).click(function(e){
                                e.preventDefault();
                                $(&quot;#cboFiscalYear_text&quot;).val($(&quot;#cboFiscalYear option:selected&quot;).text());
                                
                            })
                            $(&quot;#cboFiscalYear&quot;).change(function(e) {
                                
                                $(&quot;#cboFiscalYear_text&quot;).val($(&quot;#cboFiscalYear option:selected&quot;).text());
                            })
                            
                            function changeValAndLable(vals){
                                
                               if(vals=='1'){
                                   
                                  $(&quot;#cboPerSeat&quot;).val(1);   
                                  $(&quot;#calbased&quot;).text('Cost Per Student'); 
                                    $('#btnNo').addClass('btn-default');
                                    $('#btnNo').removeClass('btn-danger');
                                    $('#btnYes').removeClass('btn-default');
                                    $('#btnYes').addClass('btn-success');
                               } 
                               else{
                                   
                                    $(&quot;#cboPerSeat&quot;).val(0);   
                                    $(&quot;#calbased&quot;).text('Amount');   
                                     $('#btnNo').addClass('btn-danger');
                                    $('#btnNo').removeClass('btn-default');
                                    $('#btnYes').addClass('btn-default');
                                    $('#btnYes').removeClass('btn-success');
                                    
                                   
                               }
                                
                            }
                        
                        
                             Yes
                             No
                            
                            
                                
                                $(&quot;#btnNo&quot;).trigger('click');
                                $(&quot;#cboFiscalYear&quot;).trigger('click');
                                
                        
                    
                
                
                    Amount*
                    
                        
                            $
                            
                        
                        
                    
                
                                
                    Submit
                
            
        
       
        
              $('#frm_submit_accounting').validate({
                  
                  rules: {
                            cboFiscalYear: {
                                    required: true,
                                   
                            },
                            txtAmount: {
                                    required: true,
                                    number: true
                            },

                    },
                   errorPlacement: function(error, element) {
                          error.appendTo( element.closest(&quot;.elementclass&quot;).next());
                      }
              });
              
                 $(&quot;#btn_submit_accouting&quot;).click(function(e) {
                 
                    
                 
                   var myform = $('#frm_submit_accounting');

                    // Find disabled inputs, and remove the &quot;disabled&quot; attribute
                   var disabled = myform.find(':input:disabled').removeAttr('disabled');

                    $('#frm_submit_accounting').validate();
                    
                    // serialize the form
                   var serialized = myform.serialize();
                   
                    // re-disabled the set of inputs that you previously enabled
                   disabled.attr('disabled','disabled');
                   
                   e.preventDefault(); 
                  
                    if ($(&quot;#frm_submit_accounting&quot;).valid()) {
                        
                           $(&quot;#cboFiscalYear_text&quot;).val($(&quot;#cboFiscalYear option:selected&quot;).text());
                            //waitingDialog.hide();
                            //waitingDialog.show('Loading....');
                            var datastring = serialized;
                             $.ajax({
                                 type: &quot;POST&quot;,
                                 cache: false,
                                 timeout:60000,
                                 url: &quot;/PGCTechnologyServices/course-manager/addaccounting?sid=kcbtrojn&amp;time=&quot;+new Date().getTime(),
                                 data: datastring,
                                  beforeSend:function() {
                                             
                                    waitingDialog.hide();
                                    $('#ajax_instr_loader').hide();
                                    waitingDialog.show('Loading....');
                                 }, 
                                 success: function(data) {

                                     $(&quot;#accouting_table&quot;).html(data);
                                   //  resetAddInstructorForm();
                                     var magnificPopup = $.magnificPopup.instance; 
                                    // save instance in magnificPopup variable
                                     magnificPopup.close(); 
                                     //waitingDialog.hide();
                                     //var obj = jQuery.parseJSON(data); if the dataType is not specified as json uncomment this
                                     // do what ever you want with the server response
                                     showErrosIfAnySingle();
                                     waitingDialog.hide();
                                     $('#ajax_instr_loader').hide(); 
                                 },
                                  error: function (xhr, ajaxOptions, thrownError) {
                                      
                                     waitingDialog.hide();
                                     $('#ajax_instr_loader').hide(); 
                                     //alert(xhr.status);
                                     //alert(thrownError);
                                     //waitingDialog.hide();
                                   },
                                   complete: function() {
                                             
                                        waitingDialog.hide();
                                        $('#ajax_instr_loader').hide();
                                    }         

                             });
                
                       }
             });
             
        
    
    
    
    
    
    
    
        
            
                Wizard Complete!
            
             
                
                    Course Approval
                    
                        Submit for Category, then District Approval
                      
                    
                        Request District Approval 
                    
                    
                    
                        Submit
                    
                
                
                    
                    $(&quot;#btn_final_save&quot;).click(function(e) {
                                    
                            e.preventDefault(); 
                            
                             var radioValue = $(&quot;input[name='optradio']:checked&quot;).val();
                             
                            $.ajax({
                                  type: &quot;GET&quot;,
                                  cache: false,
                                  timeout:60000,
                                   url: &quot;/PGCTechnologyServices/course-manager/finalclassstatus?cgApproval=&amp;classId=0&amp;cgApprovalPosted=&quot;+radioValue+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime(),
                                   beforeSend:function() {
                                             
                                    waitingDialog.hide();
                                    $('#ajax_instr_loader').hide();
                                    waitingDialog.show('Loading....');
                                 }, 
                                  success: function(data) {
                                      
                                      //alert(data);
                                      var magnificPopup = $.magnificPopup.instance; 
                                      magnificPopup.close(); 
                                      
                                      waitingDialog.hide();
                                      $('#ajax_instr_loader').hide()

                                  },
                                   error: function (xhr, ajaxOptions, thrownError) {
                                       
                                      waitingDialog.hide();
                                      $('#ajax_instr_loader').hide()  
                                      //alert(xhr.status);
                                      //alert(thrownError);
                                      //waitingDialog.hide();
                                    },
                                    complete: function() {
                                             
                                             waitingDialog.hide();
                                             $('#ajax_instr_loader').hide();
                                         }         

                              });


                        });
                
             
            
        
    
    
    
    
    
        
            
                Total Cost Confirmation Log
            
            
                
                    
                        
                            
                                Submitter
                                Total Attended
                                Total Cost
                                Date Confirmed
                            
                        
                        
                          
                        
                    
                
            
        
    
 
        $(document).ready(function () {
            if ($(&quot;div.bhoechie-tab-content div.edit_profile_right&quot;)) {
                $(&quot;div.bhoechie-tab-content div.edit_profile_right&quot;).mCustomScrollbar({
                    theme: &quot;dark&quot;
                });
            }
            
            
        });
      
      
      /*
      $('#Num1').typeahead({
	    source:  function (query, process) {
                return $.get('/PGCTechnologyServices/course-manager/getcomponent', {comp:'num1', query: query,catid:$(&quot;#cboCategory&quot;).val() }, function (data) {
        		console.log(data);
        		data = $.parseJSON(data);
	            return process(data);
	        });
	    }
	});
        */
    
    
        
        function changeCategory(is_reload_other_pref_data){
            
            $(&quot;#Num1&quot;).val('');
            $(&quot;#Num2&quot;).val('');
            $(&quot;#Num3&quot;).val('');
            $(&quot;#is_valid_comp_num&quot;).val('');
            $(&quot;#com_num&quot;).html('');
            $('#CourseTypeCbo').empty();
              
              
                $.ajax({
                    type: &quot;GET&quot;,
                    cache: false,
                    timeout:60000,
                    url: &quot;/PGCTechnologyServices/course-manager/getcoursetypes?category=&quot; + $(&quot;#cboCategory&quot;).val()+'&amp;selected=&amp;sid=kcbtrojn&amp;is_reload_other_pref_data='+is_reload_other_pref_data+'&amp;time='+new Date().getTime(),
                    data: {},
                    dataType: &quot;JSON&quot;,
                    
                    beforeSend:function() {
                                             
                        waitingDialog.hide();
                        $('#ajax_instr_loader').hide();
                        waitingDialog.show('Loading....');
                     }, 
                    success: function(jsonData) {
                        
                    $('#CourseTypeCbo').append(jsonData.dropdown);
                   
                   catprefdata=jsonData.catprefs;
                   
                   if(is_reload_other_pref_data==1){
                       
                        if(catprefdata!=false &amp;&amp; catprefdata!=undefined){

                             $(&quot;#txtRegTerms&quot;).text(catprefdata.RegTerms);
                             $(&quot;#IsOverrideRegTerms&quot;).val(catprefdata.PromptForRegTerms);
                             
                             if(catprefdata.PromptForRegTerms=='1'){

                                $(&quot;#OverrideRegistrationTerms&quot;).show(); 
                                $(&quot;#chkmark1&quot;).removeClass().addClass('i-check fa fa-check');
                                  
                             }

                             $(&quot;#numMaxClassSize&quot;).val(catprefdata.ClassSize);
                             //$(&quot;#numMaxClassSize_opt&quot;).text(parseInt(catprefdata.ClassSize));


                            if(parseInt(catprefdata.ExtParticipants)==-1){

                                 $('#numMaxSizeExternal').val('Unlimited');
                            }
                            else{

                                 $(&quot;#numMaxSizeExternal&quot;).val(catprefdata.ExtParticipants);
                                 //$(&quot;#opt_numMaxSizeExternal&quot;).text(catprefdata.ExtParticipants);
                            }



                             if(parseInt(catprefdata.WaitListSize)==-1){

                                   $('#numMaxWaitList').val('Unlimited'); 
                                   //$('#numMaxWaitList option:eq(0)').prop('selected', true);

                             }      
                             else if(parseInt(catprefdata.WaitListSize)==0){

                                  // $('#numMaxWaitList option:eq(1)').prop('selected', true);
                                   $('#numMaxWaitList').val('No wait list');

                             }      
                             else{

                                    $('#numMaxWaitList').val(catprefdata.WaitListSize);
                                    /*$('#numMaxWaitList option:eq(2)').prop('selected', true);
                                    $('#numMaxWaitList option:eq(2)').val(catprefdata.WaitListSize);
                                    $('#numMaxWaitList option:eq(2)').text(catprefdata.WaitListSize);
                                    */

                            }
                             $(&quot;#numClassRoomHours&quot;).val(catprefdata.ClassRoomHrs);
                             //$(&quot;#numClassRoomHours&quot;).text(catprefdata.ClassRoomHrs);
                             
                               $(&quot;#txtCredits&quot;).val(catprefdata.CreditHours);

                             
                             
                                 
                                      $(&quot;#numNonClassRoomHours&quot;).trigger('change');

                                     

                             
                                 if(catprefdata.RegTerms!==undefined &amp;&amp; catprefdata.RegTerms!=null){
                                     tinymce.init({
                                             selector: '#txtRegTerms',
                                             setup: function (editor) {
                                                 editor.on('change', function () {
                                                     editor.save();
                                                 });
                                             },
                                             height: 200,
                                             width:670,
                                             menubar: false,
                                             plugins: [
                                               'advlist autolink lists charmap print preview anchor',
                                               'searchreplace visualblocks code fullscreen',
                                               'insertdatetime  table contextmenu paste code',
                                               'textcolor','link'
                                             ],
                                             toolbar: 'forecolor backcolor | fontselect fontsizeselect | undo redo | insert | styleselect | bold italic  | alignleft aligncenter alignright alignjustify | bullist numlist outdent indent link',
                                             fontsize_formats: &quot;8pt 10pt 12pt 14pt 18pt 24pt 36pt&quot;,
                                             textcolor_cols: &quot;5&quot;

                                           });

                                          tinymce.get('txtRegTerms').focus();
                                          tinymce.activeEditor.setContent(catprefdata.RegTerms);
                                      }  

                             }


                             if(catprefdata.EnrollReqMsg !== undefined){

                               $(&quot;#txtEnrollRegMsg&quot;).val(catprefdata.EnrollReqMsg);
                             }
                             if(catprefdata.DefaultPhone1 !== undefined){

                                 var DefaultPhone1='';
                                 var DefaultPhone1_='';
                                 DefaultPhone1_=catprefdata.DefaultPhone1;

                                  if(DefaultPhone1_!== undefined){

                                     var explodePhoneArr = DefaultPhone1_.split('|');
                                     for(var i = 0; i &lt; explodePhoneArr.length; i++) {
                                        if(explodePhoneArr[i]!=''){
                                         DefaultPhone1+=explodePhoneArr[i]+&quot;-&quot;;
                                         }
                                     }   
                                  }
                                 DefaultPhone1=DefaultPhone1.substring(0, DefaultPhone1.length-1);
                                 $(&quot;#txtPhone1&quot;).val(DefaultPhone1);
                             }

                              if(catprefdata.DEfaultEmail1 !== undefined){
                                     $(&quot;#txtContactEmail1&quot;).val(catprefdata.DEfaultEmail1);
                               }
                      
                        }
                        completed=true;
                        
                        waitingDialog.hide();
                        $('#ajax_instr_loader').hide();

                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                       //alert(xhr.status);
                       //alert(thrownError);
                       //waitingDialog.hide();
                       
                       waitingDialog.hide();
                        $('#ajax_instr_loader').hide();
                    },
                    complete: function() {

                        waitingDialog.hide();
                        $('#ajax_instr_loader').hide();
                    } 

             });
        }
        
        $(&quot;#numNonClassRoomHours&quot;).change(function () {
          
                                        
                                    
                    $(&quot;#txtCredits&quot;).attr('readonly','readonly');
                       
                    
                        if(parseInt($(&quot;#numNonClassRoomHours&quot;).val()) >=0){
                            
                            $(&quot;#txtCredits&quot;).val( parseInt($(&quot;#numNonClassRoomHours&quot;).val()) + parseInt($(&quot;#numClassRoomHours&quot;).val()));
                        }
                        
                        $(&quot;#txtCredits&quot;).val($(&quot;#txtCredits&quot;).val()+&quot;.00&quot;);
                        $(&quot;#txtCredits&quot;).removeAttr('readonly');
                        
                                                        
                                
          
        });
        
         $(&quot;#numClassRoomHours&quot;).change(function () {
             
                                        
                                    
                    $(&quot;#txtCredits&quot;).attr('readonly','readonly');
                       
                    
                    
                        $(&quot;#txtCredits&quot;).val( parseInt($(&quot;#numNonClassRoomHours&quot;).val()) + parseInt($(&quot;#numClassRoomHours&quot;).val()));
                        $(&quot;#txtCredits&quot;).val($(&quot;#txtCredits&quot;).val()+&quot;.00&quot;);
                        $(&quot;#txtCredits&quot;).removeAttr('readonly');
                        
                                                        
                                
          
        });
          
        $(document).ready(function () {
            
          
            
            $(&quot;#cboCategory&quot;).change(function () {
            
              changeCategory(1)

           });
           
           
           $(&quot;#Num1,#Num2,#Num3&quot;).change(function () {
               
               if($.trim($(&quot;#Num1&quot;).val())!='' &amp;&amp; $.trim($(&quot;#Num2&quot;).val())!='' &amp;&amp; $.trim($(&quot;#Num3&quot;).val())!=''){
                   
                    if($.trim($(&quot;#cboCategory&quot;).val())==''){
                            
                         //$(&quot;#Num1&quot;).val('');
                         //$(&quot;#Num2&quot;).val('');
                         //$(&quot;#Num3&quot;).val('');
                         $(&quot;#is_valid_comp_num&quot;).val('');
                         // alert(&quot;Please select Category&quot;);  
                          //waitingDialog.hide();
                    }
                    else{

                         if($.trim($(&quot;#Num1&quot;).val())!='' &amp;&amp; $.trim($(&quot;#Num2&quot;).val())!='' &amp;&amp; $.trim($(&quot;#Num3&quot;).val())!=''){

                             displayComponentName();

                         }


                    }
               
               }
               
               
           });
           
           function displayComponentName(){
           
            
             $.ajax({
                        type: &quot;GET&quot;,
                        cache: false,
                        timeout:60000,
                        url: &quot;/PGCTechnologyServices/course-manager/getcomponentname?num1=&quot;+$.trim($(&quot;#Num1&quot;).val())+&quot;&amp;num2=&quot;+$.trim($(&quot;#Num2&quot;).val())+&quot;&amp;num3=&quot;+$.trim($(&quot;#Num3&quot;).val())+&quot;&amp;catid=&quot; + $(&quot;#cboCategory&quot;).val()+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime(),
                         beforeSend:function() {
                             waitingDialog.hide();
                             $('#ajax_instr_loader').hide();
                             waitingDialog.show('Loading....');
                          }, 
                        success: function(data) {

                            var res = data.split(&quot;|&quot;);
                                if(res[0]==&quot;1&quot;){

                                   alert(res[1]); 
                                    //$(&quot;#Num1&quot;).val('');
                                    //$(&quot;#Num2&quot;).val('');
                                    //$(&quot;#Num3&quot;).val('');
                                    $(&quot;#is_valid_comp_num&quot;).val('');
                                    $(&quot;#com_num&quot;).html('');
                                }
                                else{

                                    $(&quot;#is_valid_comp_num&quot;).val('1');
                                    $(&quot;#com_num&quot;).html(res[2]);
                                    $(&quot;#MaxCredits&quot;).val(res[3]);
                                    showErrosIfAnySingle();
                                }

                           waitingDialog.hide();
                           $('#ajax_instr_loader').hide();

                        },
                         error: function (xhr, ajaxOptions, thrownError) {

                            waitingDialog.hide();
                            $('#ajax_instr_loader').hide();
                            //alert(xhr.status);
                            //alert(thrownError);
                            //waitingDialog.hide();
                          },
                          complete: function() {

                             waitingDialog.hide();
                             $('#ajax_instr_loader').hide();
                         }

                    });
            
           }
           
           function calculatDeadLines(){
               
               //waitingDialog.hide();
               //waitingDialog.show('Loading....');
               var classid=0;
                                      classid=0;
                               $.ajax({
                   type: &quot;GET&quot;,
                   cache: false,
                   timeout:60000,
                   url: &quot;/PGCTechnologyServices/course-manager/calcdeadlines?classid=&quot;+classid+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime(),
                   dataType: &quot;json&quot;,
                   beforeSend:function() {
                                             
                    waitingDialog.hide();
                    $('#ajax_instr_loader').hide();
                    waitingDialog.show('Loading....');
                 }, 
                   success: function(data) {

                       //var obj = jQuery.parseJSON(data); if the dataType is not specified as json uncomment this
                       // do what ever you want with the server response
                       
                      if(data.error==&quot;false&quot;){
                         
                         $(&quot;#internal_registration_start_date&quot;).val(data.data.dtInternalRegSDate);
                         $(&quot;#external_registration_start_date&quot;).val(data.data.dtExternalRegSDate);
                         $(&quot;#registration_deadline&quot;).val(data.data.dtRegDeadLine);
                         $(&quot;#withdrawal_deadline&quot;).val(data.data.dtWithdrawDeadLine);
                         $(&quot;#dtFollowupCompDate&quot;).val(data.data.dtFollowupCompDate);
                         //waitingDialog.hide(); 
                      }
                      else{
                          
                          alert(data.msg);
                          //waitingDialog.hide();
                          
                      }
                      //waitingDialog.hide();
                      
                      waitingDialog.hide();
                      $('#ajax_instr_loader').hide();
                       
                   },
                    error: function (xhr, ajaxOptions, thrownError) {
                        
                       
                        waitingDialog.hide();
                        $('#ajax_instr_loader').hide();
                       //alert(xhr.status);
                       //alert(thrownError);
                       //waitingDialog.hide();
                     },
                    complete: function() {

                        waitingDialog.hide();
                        $('#ajax_instr_loader').hide();
                    }          

               });
               
           }
           
           function saveDataToSession(tabindex){
               
               //waitingDialog.show('Loading....');
               //data-tabindex=&quot;1&quot;
               var finddiv=$(&quot;div&quot;).find(&quot;[data-tabindex='&quot; + tabindex + &quot;']&quot;);
               //var datastring = $(&quot;div.bhoechie-tab>div.bhoechie-tab-content&quot;).eq(tabindex).find('form').serialize();
               //var datastring = $(finddiv).find('form').serialize();
               
                if($(finddiv).find('form').attr('id')=='frm_customfields'){
                        
                        var datastring = $(finddiv).find('form').serialize();
                        
                        
                        $(finddiv).find('form').find('input[type=checkbox]').each(function(e) {
                            
                            
                            if($(this).prop(&quot;checked&quot;)!==true){
                                datastring += '&amp;'+$(this).attr('id')+'=0';
                            }
                      });
                      
                      
                    }
                    else{
                            var datastring = $(finddiv).find('form').serialize();
                     
                    }
                    
                $.ajax({
                    type: &quot;POST&quot;,
                    cache: false,
                    timeout:60000,
                    async: false,
                    url: &quot;/PGCTechnologyServices/course-manager/savedatatosession?sid=kcbtrojn&amp;time=&quot;+new Date().getTime(),
                    data: datastring,
                    dataType: &quot;json&quot;,
                    beforeSend:function() {
                                             
                        waitingDialog.hide();
                        $('#ajax_instr_loader').hide();
                        waitingDialog.show('Loading....');
                        
                     },   
                    success: function(data) {
                        
                        //waitingDialog.hide();
                        //var obj = jQuery.parseJSON(data); if the dataType is not specified as json uncomment this
                        // do what ever you want with the server response
                        
                        waitingDialog.hide();
                        $('#ajax_instr_loader').hide();
                        
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        //alert(xhr.status);
                        //alert(thrownError);
                        //waitingDialog.hide();
                    },
                    complete: function() {

                        waitingDialog.hide();
                        $('#ajax_instr_loader').hide();
                    }        
                            
                 
                });

                
               
           }
           
           
            $(&quot;div.bhoechie-tab-menu>div.list-group>div>h4>a&quot;).click(function (e) {
               
               var id=$(this).attr('id');
               if(id==&quot;general_info_main&quot;){
                   
                    $(&quot;#general_info_child&quot;).trigger('click');  
                    
               }
               else if(id==&quot;limit_main&quot;){
                   
                    $(&quot;#limit_child&quot;).trigger('click');  
               }
               else if(id==&quot;restriction_main&quot;){
                   
                    $(&quot;#restriction_child&quot;).trigger('click');  
               }
               else if(id==&quot;evaluation_main&quot;){
                   
                    $(&quot;#evaluation_child&quot;).trigger('click');  
               }
               
               
            });


             $(&quot;div.bhoechie-tab-menu>div.list-group>div>small>a&quot;).click(function (e) {
                
                e.preventDefault();
                lastIndex=$('.bhoechie-tab-menu.taller-menu div.list-group div.list-group-item.text-center.active small a.active').attr('tabindex');
                if(lastIndex==&quot;11&quot;){
                    
                    $(&quot;#is_date_generated&quot;).rules('add', 'check_schedule_dates');
                    if($.trim($(&quot;#dates_body&quot;).text())==&quot;No Schedule&quot;){
                           //$(&quot;#btn_generate_date&quot;).trigger('click'); 
                       }
                }
                 else if(lastIndex==&quot;0&quot;){
                    
                    $(&quot;#cname_&quot;).html($(&quot;#txtTitle&quot;).val());
                    
                }
                 var finddiv=$(&quot;div&quot;).find(&quot;[data-tabindex='&quot; + lastIndex + &quot;']&quot;);
               
                 //    var form=$(&quot;div.bhoechie-tab>div.bhoechie-tab-content&quot;).eq(lastIndex).find('form');
                   var form= $(finddiv).find('form');
                   
                 //var form=$(&quot;div.bhoechie-tab>div.bhoechie-tab-content&quot;).eq(lastIndex).find('form');
                    
                    var disabled = form.find(':input:disabled').removeAttr('disabled');
                    form.validate();
                    //disabled.attr('disabled','disabled');
                    if(form.valid()) {
               
                        var $this=this;
                        var index = $($this).attr('tabindex');
                        
                         $(&quot;#&quot;+lastIndex+&quot;_error&quot;).hide(); 
                         $(&quot;#&quot;+lastIndex+&quot;_ok&quot;).show(); 
                        //saveDataToSession(lastIndex);
                        
                        
                           //waitingDialog.show('Loading....');
                            //data-tabindex=&quot;1&quot;
                            var finddiv=$(&quot;div&quot;).find(&quot;[data-tabindex='&quot; + lastIndex + &quot;']&quot;);
                            //var datastring = $(&quot;div.bhoechie-tab>div.bhoechie-tab-content&quot;).eq(tabindex).find('form').serialize();
                           
                            //var datastring = $(finddiv).find('form').serialize();
                            
                           if($(finddiv).find('form').attr('id')=='frm_customfields'){
                        
                                var datastring = $(finddiv).find('form').serialize();


                                $(finddiv).find('form').find('input[type=checkbox]').each(function(e) {


                                    if($(this).prop(&quot;checked&quot;)!==true){
                                        datastring += '&amp;'+$(this).attr('id')+'=0';
                                    }
                              });


                            }
                            else{
                                    var datastring = $(finddiv).find('form').serialize();

                            }
                            disabled.attr('disabled','disabled');
                             $.ajax({
                                 type: &quot;POST&quot;,
                                 cache: false,
                                 timeout:60000,
                                 url: &quot;/PGCTechnologyServices/course-manager/savedatatosession?sid=kcbtrojn&amp;time=&quot;+new Date().getTime(),
                                 data: datastring,
                                 dataType: &quot;json&quot;,
                                 beforeSend:function() {

                                    waitingDialog.hide();
                                    $('#ajax_instr_loader').hide();
                                    waitingDialog.show('Loading....');
                                 }, 
                                 success: function(data) {

                                     //waitingDialog.hide();
                                     //var obj = jQuery.parseJSON(data); if the dataType is not specified as json uncomment this
                                     // do what ever you want with the server response
                                     
                                      if(data.is_sucess==&quot;true&quot;) {
                                          if(lastIndex==7){

                                                $(&quot;#confirm_plu&quot;).val('0');
                                          }
                                        $('.taller-menu .list-group .list-group-item small a').removeClass('active');
                                        $($this).addClass(&quot;active&quot;);
                                        $($this).parent().parent().siblings('div.active').removeClass(&quot;active&quot;);
                                        $($this).parent().parent().addClass(&quot;active&quot;);
                                        $($this).parent().parent().removeClass(&quot;selected&quot;);
                                        $($this).parent().parent().removeClass(&quot;done&quot;);
                                        $(&quot;#hdn_is_changed&quot;).val('');
                                        $(&quot;#fld_shedule&quot;).val('');

                                        $(&quot;div.bhoechie-tab>div.bhoechie-tab-content&quot;).removeClass(&quot;active&quot;);
                                        $(&quot;div.bhoechie-tab>div.bhoechie-tab-content&quot;).removeClass(&quot;selected&quot;);
                                        $(&quot;div.bhoechie-tab>div.bhoechie-tab-content&quot;).removeClass(&quot;done&quot;);
                                        //$(&quot;div.bhoechie-tab>div.bhoechie-tab-content&quot;).eq(index).addClass(&quot;active&quot;);

                                            if(index==0){
                                              
                                              $(&quot;#btnNextStep&quot;).show();
                                            }
                                            if(index>0){
                         
                                                    $(&quot;#btnPrevStep&quot;).show();
                                                    $(&quot;#btnNextStep&quot;).show();
                                                    $(&quot;#buttonFinish&quot;).hide();
                                                }
                                               else{
                                                   
                                                    $(&quot;#btnPrevStep&quot;).hide();
                                                    $(&quot;#buttonFinish&quot;).hide();
                                               }
                                               
                                               if(index==maxTabs){

                                                   $(&quot;#buttonFinish&quot;).show();
                                                    $(&quot;#btnNextStep&quot;).hide();
                                               }
                                               
                                         if(index==&quot;12&quot;){
                                             
                                             $(&quot;#startdate_deadline&quot;).val($(&quot;#generate_start_date&quot;).val()); 
                                             $(&quot;#enddate_deadline&quot;).val($(&quot;#generate_end_date&quot;).val()); 
                                        
                                            if(($(&quot;#internal_registration_start_date&quot;).val()==&quot;&quot; || $(&quot;#internal_registration_start_date&quot;).val()==&quot;__/__/____&quot;) || ($(&quot;#internal_registration_start_date&quot;).val()==&quot;&quot; || $(&quot;#internal_registration_start_date&quot;).val()==&quot;__/__/____&quot;)){
                                                
                                                calculatDeadLines();
                                            } 
                                              else if(dateChange==true){
                                                        
                                                         var r = confirm(&quot;Would you like to re-calculate course deadlines?&quot;);
                                                         if (r == true) {
                                                             
                                                            calculatDeadLines();
                                                        }  
                                                        dateChange=false;
                                                    }
                                         }
                                        else if(index==&quot;16&quot;){
                                            loadTracks();
                                         } 
                                        $(&quot;div&quot;).find(&quot;[data-tabindex='&quot; + index + &quot;']&quot;).addClass(&quot;active&quot;);
                                        $('.taller-menu .list-group .list-group-item small a').find(&quot;[tabindex='&quot; + index + &quot;']&quot;).addClass('active');

                                        $($this).parent().parent().prevAll().addClass(&quot;selected&quot;);
                                        $($this).parent().parent().prevAll().removeClass(&quot;done&quot;);
                                        $($this).parent().parent().nextAll().removeClass(&quot;selected&quot;);
                                        $($this).parent().parent().nextAll().addClass(&quot;done&quot;);
                                        //waitingDialog.hide();
                                        
                                        }
                                        else if(data.type!==undefined &amp;&amp; data.type=='confirm' &amp;&amp; data.newconfirm!==undefined &amp;&amp; $(&quot;#confirm_plu&quot;).val()=='0'){
                                    
                                            var r = confirm(data.error);
                                            if (r == true) {

                                                if(lastIndex==7){

                                                      $(&quot;#confirm_plu&quot;).val('1');
                                                }


                                            }
                                            else{

                                                $(&quot;#txtCredits&quot;).val(data.old_plu);
                                                $(&quot;#numClassRoomHours&quot;).val(data.old_class_room_hours);
                                                //$(&quot;#numNonClassRoomHours&quot;).val(data.old_non_class_room_hours);
                                            }

                                         }
                                        else if(data.type!==undefined &amp;&amp; data.newconfirm===undefined ){
                                    
                                            var r = confirm(data.error);
                                            if (r == true) {
                                                //alert(lastIndex);
                                                if(lastIndex==7){

                                                      $.ajax({
                                                                type: &quot;POST&quot;,
                                                                cache: false,
                                                                timeout:60000,
                                                                url: &quot;/PGCTechnologyServices/course-manager/processedroster?classId=0&amp;numMaxClassSize=&quot;+$(&quot;#numMaxClassSize&quot;).val()+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime(),
                                                                data: datastring,
                                                                dataType: &quot;json&quot;,
                                                                beforeSend:function() {

                                                                    waitingDialog.hide();
                                                                    $('#ajax_instr_loader').hide();
                                                                    waitingDialog.show('Loading....');
                                                                 },
                                                                success: function(data) {
                                                                    
                                                                    waitingDialog.hide();
                                                                    $('#ajax_instr_loader').hide();

                                                                },
                                                                complete: function() {
                                             
                                                                    waitingDialog.hide();
                                                                    $('#ajax_instr_loader').hide();
                                                                } 
                                                         })


                                                }


                                            }  

                                         }
                                        else{
                                            
                                             alert(data.error);
                                        }
                                     
                                     //waitingDialog.hide();
                                     
                                 },
                                  error: function (xhr, ajaxOptions, thrownError) {
                                  
                                     waitingDialog.hide();
                                     $('#ajax_instr_loader').hide();   
                                     //alert(xhr.status);
                                     //alert(thrownError);
                                     //waitingDialog.hide();
                                   },
                                   complete: function() {
                                             
                                        waitingDialog.hide();
                                        $('#ajax_instr_loader').hide();
                                         
                                    }  

                             });
                        
                      
                        
                 }
                 else{
                 
                    $(&quot;#&quot;+lastIndex+&quot;_error&quot;).show();   
                    $(&quot;#&quot;+lastIndex+&quot;_ok&quot;).hide();  
                    disabled.attr('disabled','disabled');
                 }
            });
            
           


            $('#generate_start_date').datetimepicker({
                lang: 'en',
                format: 'm/d/Y h:i A',
                formatTime: 'h:i A',
                value: '07/07/2020 08:00 AM',
                step: 10,
                closeOnDateSelect:true,
                onSelectDate:function (dateText, inst) {
        
                    /*if($(&quot;#dates_body&quot;).text()!=&quot;No Schedule&quot;){

                            var r = confirm(&quot;Are you sure you want to override previously generated dates?&quot;);
                            if (r == true) {

                               $(&quot;#btn_generate_date&quot;).trigger('click'); 
                               dateChange=true;

                            }
                            else{

                                jQuery('#generate_start_date').focus();
                                jQuery('#generate_end_date').datetimepicker('hide');
                                document.selection.empty();

                                return false;
                            }

                        }*/
                }
            });
            
            
            //  $('#generate_start_date').datetimepicker({ mouseWheel: false,inverseMouseWheel: false,mousewheelYearsLine:false,scrollMonth:false,scrollInput:false });

            $('#generate_end_date').datetimepicker({
                  lang: 'en',
                format: 'm/d/Y h:i A',
                formatTime: 'h:i A',
                value: '07/07/2020 09:00 AM',
                step: 10,
                closeOnDateSelect:true,
                onSelectDate:function (dateText, inst) {
        
             /*if($(&quot;#dates_body&quot;).text()!=&quot;No Schedule&quot;){
                                              
                    var r = confirm(&quot;Are you sure you want to override previously generated dates?&quot;);
                    if (r == true) {

                       $(&quot;#btn_generate_date&quot;).trigger('click'); 
                       dateChange=true;

                    }
                    else{

                         jQuery('#generate_start_date').datetimepicker('hide');
                         jQuery('#generate_end_date').focus();
                        return false;
                    }

                 }*/
    } });
            

            $('#internal_registration_start_date').datetimepicker({
                //dayOfWeekStart: 1,
                lang: 'en',
                timepicker: false,
                format: 'm/d/Y',
                mouseWheel: false,
                inverseMouseWheel: false,
                mousewheelYearsLine:false,
                scrollMonth:false,
                scrollInput:false,
                closeOnDateSelect:true
            });
            $('#internal_registration_start_date').datetimepicker({ value: '', step: 10,mouseWheel: false,inverseMouseWheel: false,mousewheelYearsLine:false,scrollMonth:false,scrollInput:false,closeOnDateSelect:true });
            //$('#internal_registration_start_date').datetimepicker({ mouseWheel: false,inverseMouseWheel: false,mousewheelYearsLine:false,scrollMonth:false,scrollInput:false });

            $('#external_registration_start_date').datetimepicker({
                //dayOfWeekStart: 1,
                lang: 'en',
                timepicker: false,
                format: 'm/d/Y',
                mouseWheel: false,
                inverseMouseWheel: false,
                mousewheelYearsLine:false,
                scrollMonth:false,
                scrollInput:false,
                closeOnDateSelect:true
            });
            $('#external_registration_start_date').datetimepicker({ value: '', step: 10,mouseWheel: false,inverseMouseWheel: false,mousewheelYearsLine:false,scrollMonth:false,scrollInput:false });
            //$('#external_registration_start_date').datetimepicker({ mouseWheel: false,inverseMouseWheel: false,mousewheelYearsLine:false,scrollMonth:false,scrollInput:false });



            $('#dtFollowupCompDate').datetimepicker({
                //dayOfWeekStart: 1,
                lang: 'en',
                timepicker: false,
                format: 'm/d/Y',
                formatDate: 'm/d/Y',
                mouseWheel: false,
                inverseMouseWheel: false,
                mousewheelYearsLine:false,
                scrollMonth:false,
                scrollInput:false,
                closeOnDateSelect:true
            });
            $('#dtFollowupCompDate').datetimepicker({ value: '', step: 10,mouseWheel: false,inverseMouseWheel: false,mousewheelYearsLine:false,scrollMonth:false,scrollInput:false });
            //$('#dtFollowupCompDate').datetimepicker({ mouseWheel: false,inverseMouseWheel: false,mousewheelYearsLine:false,scrollMonth:false,scrollInput:false });
            
            $('#registration_deadline').datetimepicker({
                //dayOfWeekStart: 1,
                lang: 'en',
                timepicker: false,
                format: 'm/d/Y',
                formatDate: 'm/d/Y',
                mouseWheel: false,
                inverseMouseWheel: false,
                mousewheelYearsLine:false,
                scrollMonth:false,
                scrollInput:false,
                closeOnDateSelect:true
            });
            $('#registration_deadline').datetimepicker({ value: '', step: 10,mouseWheel: false,inverseMouseWheel: false,mousewheelYearsLine:false,scrollMonth:false,scrollInput:false,closeOnDateSelect:true });
            //$('#registration_deadline').datetimepicker({ mouseWheel: false,inverseMouseWheel: false,mousewheelYearsLine:false,scrollMonth:false,scrollInput:false });

            $('#withdrawal_deadline').datetimepicker({
                //dayOfWeekStart: 1,
                lang: 'en',
                timepicker: false,
                format: 'm/d/Y',
                formatDate: 'm/d/Y',
                mouseWheel: false,
                inverseMouseWheel: false,
                mousewheelYearsLine:false,
                scrollMonth:false,
                scrollInput:false,
                closeOnDateSelect:true
            });
            $('#withdrawal_deadline').datetimepicker({ value: '', step: 10,mouseWheel: false,inverseMouseWheel: false,mousewheelYearsLine:false,scrollMonth:false,scrollInput:false,closeOnDateSelect:true });
            //$('#withdrawal_deadline').datetimepicker({ mouseWheel: false,inverseMouseWheel: false,mousewheelYearsLine:false,scrollMonth:false,scrollInput:false });

            $('#course_period_date').datetimepicker({
                //dayOfWeekStart: 1,
                lang: 'en',
                timepicker: false,
                format: 'm/d/Y',
                formatDate: 'm/d/Y',
                mouseWheel: false,
                inverseMouseWheel: false,
                mousewheelYearsLine:false,
                scrollMonth:false,
                scrollInput:false,
                closeOnDateSelect:true
            });
           // $('#course_period_date').datetimepicker({ value: '02/07/2017', step: 10 });
            $('#course_period_date').datetimepicker({ mouseWheel: false,inverseMouseWheel: false,mousewheelYearsLine:false,scrollMonth:false,scrollInput:false,closeOnDateSelect:true });

            $('#dtExpiryDate').datetimepicker({
                //dayOfWeekStart: 1,
                lang: 'en',
                timepicker: false,
                format: 'm/d/Y',
                formatDate: 'm/d/Y',
                mouseWheel: false,
                inverseMouseWheel: false,
                mousewheelYearsLine:false,
                scrollMonth:false,
                scrollInput:false,
                closeOnDateSelect:true
            });
            $('#dtExpiryDate').datetimepicker({ value:'', step: 10,mouseWheel: false,inverseMouseWheel: false,mousewheelYearsLine:false,scrollMonth:false,scrollInput:false,closeOnDateSelect:true });

            /*$('#date_updated').datetimepicker({
                dayOfWeekStart: 1,
                lang: 'en',
                timepicker: false,
                format: 'm/d/Y',
                formatDate: 'm/d/Y',
                mouseWheel: false,
                inverseMouseWheel: false,
                mousewheelYearsLine:false,
                scrollMonth:false,scrollInput:false
            });
            $('#date_updated').datetimepicker({ value: '02/07/2017', step: 10,mouseWheel: false,inverseMouseWheel: false,mousewheelYearsLine:false,scrollMonth:false,scrollInput:false });
            */
            $('#DTRsvSeat').datetimepicker({
                //dayOfWeekStart: 1,
                lang: 'en',
                timepicker: false,
                format: 'm/d/Y',
                formatDate: 'm/d/Y',
                step: 10,
                mouseWheel: false,
                inverseMouseWheel: false,
                mousewheelYearsLine:false,
                scrollMonth:false,scrollInput:false,
                closeOnDateSelect:true
            });

            $('#course_start_time').datetimepicker({
                datepicker: false,
                format: 'h:i A',
                formatTime: 'h:i A',
                step: 10,
                mouseWheel: false,
                inverseMouseWheel: false,
                mousewheelYearsLine:false,
                scrollMonth:false,scrollInput:false,
                closeOnDateSelect:true
            });
            $('#course_start_time').datetimepicker({ mouseWheel: false,inverseMouseWheel: false,mousewheelYearsLine:false,scrollMonth:false,scrollInput:false,closeOnDateSelect:true });

            $('#course_end_time').datetimepicker({
                datepicker: false,
                format: 'h:i A',
                formatTime: 'h:i A',
                step: 10,
                mouseWheel: false,
                inverseMouseWheel: false,
                mousewheelYearsLine:false,
                scrollMonth:false,scrollInput:false,
                closeOnDateSelect:true
            });
            $('#course_end_time').datetimepicker({ mouseWheel: false,inverseMouseWheel: false,mousewheelYearsLine:false,scrollMonth:false,scrollInput:false,closeOnDateSelect:true });

             
            
            $('#btnPrevStep').click(function () {
                
                lastIndex=$('.bhoechie-tab-menu.taller-menu div.list-group div.list-group-item.text-center.active small a.active').attr('tabindex');
                 // var form=$(&quot;div.bhoechie-tab>div.bhoechie-tab-content&quot;).eq(lastIndex).find('form');
                 if(lastIndex==&quot;11&quot;){
                    
                     $(&quot;#is_date_generated&quot;).rules('add', 'check_schedule_dates');
                     if($.trim($(&quot;#dates_body&quot;).text())==&quot;No Schedule&quot;){
                          // $(&quot;#btn_generate_date&quot;).trigger('click'); 
                       }
                 }
                
                 else if(lastIndex==&quot;0&quot;){
                    
                    $(&quot;#cname_&quot;).html($(&quot;#txtTitle&quot;).val());
                    
                }
                   var form=$(&quot;div&quot;).find(&quot;[data-tabindex='&quot; + lastIndex + &quot;']&quot;).find('form');
                    
                    var disabled = form.find(':input:disabled').removeAttr('disabled');
                    form.validate();
                    //disabled.attr('disabled','disabled');
                    if(form.valid()) {
                       
                            $(&quot;#&quot;+lastIndex+&quot;_error&quot;).hide(); 
                            $(&quot;#&quot;+lastIndex+&quot;_ok&quot;).show(); 
                           //waitingDialog.show('Loading....');
                            //data-tabindex=&quot;1&quot;
                            var finddiv=$(&quot;div&quot;).find(&quot;[data-tabindex='&quot; + lastIndex + &quot;']&quot;);
                            //var datastring = $(&quot;div.bhoechie-tab>div.bhoechie-tab-content&quot;).eq(tabindex).find('form').serialize();
                            //var datastring = $(finddiv).find('form').serialize();
                            
                            
                             if($(finddiv).find('form').attr('id')=='frm_customfields'){
                        
                                var datastring = $(finddiv).find('form').serialize();


                                $(finddiv).find('form').find('input[type=checkbox]').each(function(e) {


                                    if($(this).prop(&quot;checked&quot;)!==true){
                                        datastring += '&amp;'+$(this).attr('id')+'=0';
                                    }
                              });


                            }
                            else{
                                    var datastring = $(finddiv).find('form').serialize();

                            }
                            
                            disabled.attr('disabled','disabled');
                             $.ajax({
                                 type: &quot;POST&quot;,
                                 cache: false,
                                 timeout:60000,
                                 url: &quot;/PGCTechnologyServices/course-manager/savedatatosession?sid=kcbtrojn&amp;time=&quot;+new Date().getTime(),
                                 data: datastring,
                                 dataType: &quot;json&quot;,
                                  beforeSend:function() {
                                             
                                    waitingDialog.hide();
                                    $('#ajax_instr_loader').hide();
                                    waitingDialog.show('Loading....');
                                 }, 
                                 success: function(data) {

                                     //waitingDialog.hide();
                                     
                                    if(data.is_sucess==&quot;true&quot;) {
                                        
                                        if(lastIndex==7){
                                            $(&quot;#confirm_plu&quot;).val('0');
                                          }
                                        //    var index = $('div.edit_profile_cont>div.active').index();
                                            var index = lastIndex;
                                            
                                            if (index > 0) {
                                                index = index - 1;
                                            }
                                            
                                            
                                            while ($(&quot;div&quot;).find(&quot;[data-tabindex='&quot; + index + &quot;']&quot;).length&lt;=0) {
                                                index=index-1;
                                             }
                                            
                                            $(&quot;#hdn_is_changed&quot;).val('');
                                            $(&quot;#fld_shedule&quot;).val('');
                                            $(&quot;div.bhoechie-tab>div.bhoechie-tab-content&quot;).removeClass(&quot;active&quot;);
                                            $('.taller-menu .list-group .list-group-item small a').removeClass('active');
                                            $('.taller-menu .list-group .list-group-item small a[tabindex=' + index + ']').addClass(&quot;active&quot;);
                                            //$(&quot;div.bhoechie-tab>div.bhoechie-tab-content [data-tabindex='&quot; + index + &quot;']'&quot;).addClass(&quot;active&quot;);
                                            $(&quot;div&quot;).find(&quot;[data-tabindex='&quot; + index + &quot;']&quot;).addClass(&quot;active&quot;);
                                            $('.taller-menu .list-group .list-group-item small a').find(&quot;[tabindex='&quot; + index + &quot;']&quot;).addClass('active');
                                           
                                            var tabidx= $(&quot;div.bhoechie-tab>div.bhoechie-tab-content.active&quot;).attr('data-tabindex');
                                            
                                            if(tabidx==0){
                                              
                                              $(&quot;#btnNextStep&quot;).show();
                                            }
                                             if(tabidx>0){
                         
                                                    $(&quot;#btnPrevStep&quot;).show();
                                                     $(&quot;#btnNextStep&quot;).show();
                                                     $(&quot;#buttonFinish&quot;).hide();
                                                }
                                               else{
                                                   
                                                    $(&quot;#btnPrevStep&quot;).hide();
                                                    $(&quot;#buttonFinish&quot;).hide();
                                               } 
                                               if(tabidx==18){

                                                   $(&quot;#buttonFinish&quot;).show();
                                                    $(&quot;#btnNextStep&quot;).hide();
                                               }
                                             if(tabidx==&quot;12&quot;){
                                                 
                                                 $(&quot;#startdate_deadline&quot;).val($(&quot;#generate_start_date&quot;).val()); 
                                                 $(&quot;#enddate_deadline&quot;).val($(&quot;#generate_end_date&quot;).val()); 
                                       
                                                if(($(&quot;#internal_registration_start_date&quot;).val()==&quot;&quot; || $(&quot;#internal_registration_start_date&quot;).val()==&quot;__/__/____&quot;) || ($(&quot;#internal_registration_start_date&quot;).val()==&quot;&quot; || $(&quot;#internal_registration_start_date&quot;).val()==&quot;__/__/____&quot;)){

                                                    calculatDeadLines();
                                                } 
                                                  else if(dateChange==true){
                                                        
                                                         var r = confirm(&quot;Would you like to re-calculate course deadlines?&quot;);
                                                         if (r == true) {
                                                             
                                                            calculatDeadLines();
                                                        }  
                                                        dateChange=false;
                                                    }
                                             }
                                             else if(tabidx==&quot;16&quot;){

                                                loadTracks();

                                            }

                                            $('.taller-menu .list-group .list-group-item small a').find(&quot;[tabindex='&quot; + tabidx + &quot;']&quot;).addClass('active');
                                            SelectActiveTab(tabidx);
                                            $(&quot;a[tabindex='&quot; + tabidx + &quot;']&quot;).addClass('active');
                                            $(&quot;a[tabindex='&quot; + lastIndex + &quot;']&quot;).removeClass('active');
                                     
                                        }
                                        else if(data.type!==undefined &amp;&amp; data.type=='confirm' &amp;&amp; data.newconfirm!==undefined &amp;&amp; $(&quot;#confirm_plu&quot;).val()=='0'){
                                    
                                            var r = confirm(data.error);
                                            if (r == true) {

                                                if(lastIndex==7){

                                                      $(&quot;#confirm_plu&quot;).val('1');
                                                }


                                            }
                                            else{

                                                $(&quot;#txtCredits&quot;).val(data.old_plu);
                                                $(&quot;#numClassRoomHours&quot;).val(data.old_class_room_hours);
                                                //$(&quot;#numNonClassRoomHours&quot;).val(data.old_non_class_room_hours);
                                            }

                                         }
                                        else if(data.type!==undefined &amp;&amp; data.newconfirm===undefined ){
                                    
                                            var r = confirm(data.error);
                                            if (r == true) {
                                                //alert(lastIndex);
                                                if(lastIndex==7){

                                                      $.ajax({
                                                                type: &quot;POST&quot;,
                                                                cache: false,
                                                                timeout:60000,
                                                                url: &quot;/PGCTechnologyServices/course-manager/processedroster?classId=0&amp;numMaxClassSize=&quot;+$(&quot;#numMaxClassSize&quot;).val()+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime(),
                                                                data: datastring,
                                                                dataType: &quot;json&quot;,
                                                                beforeSend:function() {

                                                                    waitingDialog.hide();
                                                                    $('#ajax_instr_loader').hide();
                                                                    waitingDialog.show('Loading....');
                                                                 },
                                                                success: function(data) {
                                                                    
                                                                    waitingDialog.hide();
                                                                    $('#ajax_instr_loader').hide();

                                                                },
                                                                complete: function() {
                                             
                                                                    waitingDialog.hide();
                                                                    $('#ajax_instr_loader').hide();
                                                                } 
                                                         })


                                                }


                                            }  

                                         }
                                        else{
                                            
                                             alert(data.error);
                                        }
                                        
                                     waitingDialog.hide();
                                    $('#ajax_instr_loader').hide();    
                                 },
                                  error: function (xhr, ajaxOptions, thrownError) {
                                      
                                     waitingDialog.hide();
                                     $('#ajax_instr_loader').hide(); 
                                     //alert(xhr.status);
                                     //alert(thrownError);
                                     //waitingDialog.hide();
                                 },
                                 complete: function() {

                                   waitingDialog.hide();
                                   $('#ajax_instr_loader').hide();
                                }

                             });
                        //saveDataToSession(lastIndex);               

                      
                        
                    }
                    else{
                    
                          $(&quot;#&quot;+lastIndex+&quot;_error&quot;).show();   
                          $(&quot;#&quot;+lastIndex+&quot;_ok&quot;).hide();   
                          disabled.attr('disabled','disabled');
                          
                          var index = $('div.edit_profile_cont>div.active').index();
                            if (index > 0) {
                                index = index - 1;
                            }
                            $(&quot;div.bhoechie-tab>div.bhoechie-tab-content&quot;).removeClass(&quot;active&quot;);
                            $(&quot;div.bhoechie-tab>div.bhoechie-tab-content&quot;).eq(index).addClass(&quot;active&quot;);
                            $('.taller-menu .list-group .list-group-item small a').removeClass('active');
                            $('.taller-menu .list-group .list-group-item small a[tabindex=' + index + ']').addClass(&quot;active&quot;);
                            $('.taller-menu .list-group .list-group-item small a').find(&quot;[tabindex='&quot; + index + &quot;']&quot;).addClass('active');

                            var tabidx= $(&quot;div.bhoechie-tab-content.active&quot;).attr('data-tabindex');

                            if(tabidx==0){

                              $(&quot;#btnNextStep&quot;).show();
                            }
                             if(tabidx>0){

                                    $(&quot;#btnPrevStep&quot;).show();
                                     $(&quot;#btnNextStep&quot;).show();
                                }
                               else{

                                    $(&quot;#btnPrevStep&quot;).hide();
                                    $(&quot;#buttonFinish&quot;).hide();
                               } 
                               if(tabidx==maxTabs){

                                   $(&quot;#buttonFinish&quot;).show();
                                    $(&quot;#btnNextStep&quot;).hide();
                               }
                             if(tabidx==&quot;12&quot;){

                                 $(&quot;#startdate_deadline&quot;).val($(&quot;#generate_start_date&quot;).val()); 
                                 $(&quot;#enddate_deadline&quot;).val($(&quot;#generate_end_date&quot;).val()); 

                                if(($(&quot;#internal_registration_start_date&quot;).val()==&quot;&quot; || $(&quot;#internal_registration_start_date&quot;).val()==&quot;__/__/____&quot;) || ($(&quot;#internal_registration_start_date&quot;).val()==&quot;&quot; || $(&quot;#internal_registration_start_date&quot;).val()==&quot;__/__/____&quot;)){

                                    calculatDeadLines();
                                } 
                                  else if(dateChange==true){

                                         var r = confirm(&quot;Would you like to re-calculate course deadlines?&quot;);
                                         if (r == true) {

                                            calculatDeadLines();
                                        }  
                                        dateChange=false;
                                    }
                             }
                             else if(tabidx==&quot;16&quot;){

                                loadTracks();

                            }

                            $('.taller-menu .list-group .list-group-item small a').find(&quot;[tabindex='&quot; + tabidx + &quot;']&quot;).addClass('active');
                            SelectActiveTab(tabidx);
                            $(&quot;a[tabindex='&quot; + tabidx + &quot;']&quot;).addClass('active');
                            $(&quot;a[tabindex='&quot; + lastIndex + &quot;']&quot;).removeClass('active');

                    }

                return false;
            });
            
            
            
                            $('#buttonFinish').click(function () {

                     $.ajax({
                                    type: &quot;POST&quot;,
                                    cache: false,
                                    timeout:60000,
                                    url: &quot;/PGCTechnologyServices/course-manager/finishddcourse?sid=kcbtrojn&amp;time=&quot;+new Date().getTime(),
                                    dataType: &quot;json&quot;,
                                    beforeSend:function() {
                                             
                                        waitingDialog.hide();
                                        $('#ajax_instr_loader').hide();
                                        waitingDialog.show('Loading....');
                                     },
                                    success: function(data) {

                                          $(&quot;#hdn_is_changed&quot;).val('');   
                                          $(&quot;#fld_shedule&quot;).val('');   
                                         if(data.is_sucess==&quot;false&quot;){
                                            
                                            alert(&quot;Error while creating course&quot;);
                                            
                                            }
                                            else{

                                                finish_success=true;
                                                window.location.href='/PGCTechnologyServices/course-manager/update?class_id='+data.course_id+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime();
                                            }
                                       
                                        waitingDialog.hide();
                                        $('#ajax_instr_loader').hide();
                                        
                                    },
                                    complete: function() {
                                             
                                        waitingDialog.hide();
                                        $('#ajax_instr_loader').hide();
                                    }     
                        })

                })
                    
            $('#btnNextStep').click(function () {
                lastIndex=$('.bhoechie-tab-menu.taller-menu div.list-group div.list-group-item.text-center.active small a.active').attr('tabindex');
                 
                if(lastIndex==&quot;11&quot;){
                    
                     if($.trim($(&quot;#dates_body&quot;).text())==&quot;No Schedule&quot;){
                         //  $(&quot;#btn_generate_date&quot;).trigger('click'); 
                       }
                }
                else if(lastIndex==&quot;0&quot;){
                    
                    $(&quot;#cname_&quot;).html($(&quot;#txtTitle&quot;).val());
                    
                }
                
                
               // saveDataToSession(lastIndex);    
                //var form=$(&quot;div.bhoechie-tab>div.bhoechie-tab-content&quot;).eq(lastIndex).find('form');
                var form=$(&quot;div&quot;).find(&quot;[data-tabindex='&quot; + lastIndex + &quot;']&quot;).find('form');
                var disabled = form.find(':input:disabled').removeAttr('disabled');
                form.validate();
                //disabled.attr('disabled','disabled');
                if(form.valid()) {

                     $(&quot;#&quot;+lastIndex+&quot;_error&quot;).hide(); 
                     $(&quot;#&quot;+lastIndex+&quot;_ok&quot;).show(); 
                     var index = $('div.edit_profile_cont>div.active').index();
                     
                   

                    //if (index &lt; $('div.edit_profile_cont>div.bhoechie-tab-content:last-child').index()) {
                    //    index = index + 1;
                    //}
                    if (index &lt; 18) {
                        index = index + 1;
                    }

                     //waitingDialog.show('Loading....');
                    //data-tabindex=&quot;1&quot;
                    var finddiv=$(&quot;div&quot;).find(&quot;[data-tabindex='&quot; + lastIndex + &quot;']&quot;);
                    //var datastring = $(&quot;div.bhoechie-tab>div.bhoechie-tab-content&quot;).eq(tabindex).find('form').serialize();
                    if($(finddiv).find('form').attr('id')=='frm_customfields'){
                        
                        var datastring = $(finddiv).find('form').serialize();


                        $(finddiv).find('form').find('input[type=checkbox]').each(function(e) {


                            if($(this).prop(&quot;checked&quot;)!==true){
                                datastring += '&amp;'+$(this).attr('id')+'=0';
                            }
                      });


                    }
                    else{
                            var datastring = $(finddiv).find('form').serialize();

                    }   
                    disabled.attr('disabled','disabled');
                     $.ajax({
                         type: &quot;POST&quot;,
                         cache: false,
                         timeout:60000,
                         url: &quot;/PGCTechnologyServices/course-manager/savedatatosession?sid=kcbtrojn&amp;time=&quot;+new Date().getTime(),
                         data: datastring,
                         dataType: &quot;json&quot;,
                         beforeSend:function() {
                                             
                            waitingDialog.hide();
                            $('#ajax_instr_loader').hide();
                            waitingDialog.show('Loading....');
                         },
                         success: function(data) {

                             //waitingDialog.hide();
                             
                            if(data.is_sucess==&quot;true&quot;) {
                                    
                                    if(lastIndex==7){
                                      $(&quot;#confirm_plu&quot;).val('0');
                                    }
                                    $(&quot;#hdn_is_changed&quot;).val('');
                                    $(&quot;#fld_shedule&quot;).val('');
                                    $(&quot;div.bhoechie-tab>div.bhoechie-tab-content&quot;).removeClass(&quot;active&quot;);
                                    $(&quot;div.bhoechie-tab>div.bhoechie-tab-content&quot;).eq(index).addClass(&quot;active&quot;);
                                    $('.taller-menu .list-group .list-group-item small a').removeClass('active');
                                    $('.taller-menu .list-group .list-group-item small a[tabindex=' + index + ']').addClass(&quot;active&quot;);
                                    $('.taller-menu .list-group .list-group-item small a').find(&quot;[tabindex='&quot; + index + &quot;']&quot;).addClass('active');
                                    
                                    var tabidx= $(&quot;div.bhoechie-tab-content.active&quot;).attr('data-tabindex');
                                    
                                     if(tabidx==0){
                                         
                                         $(&quot;#btnNextStep&quot;).show();
                                     }
                                     if(tabidx>0){
                         
                                            $(&quot;#btnPrevStep&quot;).show();
                                             $(&quot;#btnNextStep&quot;).show();
                                        }
                                       else{

                                            $(&quot;#btnPrevStep&quot;).hide();
                                            $(&quot;#buttonFinish&quot;).hide();
                                       } 
                                       if(tabidx==maxTabs){

                                           $(&quot;#buttonFinish&quot;).show();
                                            $(&quot;#btnNextStep&quot;).hide();
                                       }
                    
                                     if(tabidx==&quot;12&quot;){
                                         
                                        $(&quot;#startdate_deadline&quot;).val($(&quot;#generate_start_date&quot;).val()); 
                                        $(&quot;#enddate_deadline&quot;).val($(&quot;#generate_end_date&quot;).val()); 
                                        
                                        
                                        if(($(&quot;#internal_registration_start_date&quot;).val()==&quot;&quot; || $(&quot;#internal_registration_start_date&quot;).val()==&quot;__/__/____&quot;) || ($(&quot;#external_registration_start_date&quot;).val()==&quot;&quot; || $(&quot;#external_registration_start_date&quot;).val()==&quot;__/__/____&quot;)){
                                            
                                            calculatDeadLines();
                                        } 
                                          else if(dateChange==true){
                                                        
                                                var r = confirm(&quot;Would you like to re-calculate course deadlines?&quot;);
                                                if (r == true) {

                                                   calculatDeadLines();
                                               }  
                                               dateChange=false;
                                           }
                                     }
                                     else if(tabidx==&quot;16&quot;){
                                         
                                        
                                        loadTracks();
                                        
                                     }
                     
                                    $('.taller-menu .list-group .list-group-item small a').find(&quot;[tabindex='&quot; + tabidx + &quot;']&quot;).addClass('active');
                                     SelectActiveTab(tabidx);
                                      $(&quot;a[tabindex]&quot;).removeClass('active');
                                     $(&quot;a[tabindex='&quot; + tabidx + &quot;']&quot;).addClass('active');
                                     $(&quot;a[tabindex='&quot; + lastIndex + &quot;']&quot;).removeClass('active');

                                }
                               else if(data.type!==undefined &amp;&amp; data.type=='confirm' &amp;&amp; data.newconfirm!==undefined &amp;&amp; $(&quot;#confirm_plu&quot;).val()=='0'){
                                    
                                        var r = confirm(data.error);
                                        if (r == true) {
                                            
                                            if(lastIndex==7){
                                                  
                                                  $(&quot;#confirm_plu&quot;).val('1');
                                            }


                                        }
                                        else{

                                            $(&quot;#txtCredits&quot;).val(data.old_plu);
                                            $(&quot;#numClassRoomHours&quot;).val(data.old_class_room_hours);
                                            //$(&quot;#numNonClassRoomHours&quot;).val(data.old_non_class_room_hours);
                                        }

                                 }
                                else if(data.type!==undefined &amp;&amp; data.newconfirm===undefined ){
                                    
                                    var r = confirm(data.error);
                                    if (r == true) {
                                       
                                        if(lastIndex==7){
                                            
                                              $.ajax({
                                                        type: &quot;POST&quot;,
                                                        cache: false,
                                                        timeout:60000,
                                                        url: &quot;/PGCTechnologyServices/course-manager/processedroster?classId=0&amp;numMaxClassSize=&quot;+$(&quot;#numMaxClassSize&quot;).val()+'&amp;sid=kcbtrojn&amp;time='+new Date().getTime(),
                                                        data: datastring,
                                                        dataType: &quot;json&quot;,
                                                        beforeSend:function() {
                                             
                                                            waitingDialog.hide();
                                                            $('#ajax_instr_loader').hide();
                                                            waitingDialog.show('Loading....');
                                                         }, 
                                                        success: function(data) {
                                                            
                                                            waitingDialog.hide();
                                                            $('#ajax_instr_loader').hide();
                                                            
                                                        },
                                                         complete: function() {
                                             
                                                            waitingDialog.hide();
                                                            $('#ajax_instr_loader').hide();
                                                        }
                                                 })
                                            
                                            
                                        }
                                        
                                        
                                    }  
                                         
                                 }
                                else{

                                     alert(data.error);
                                }
                                
                                
                             waitingDialog.hide();
                            $('#ajax_instr_loader').hide();
                            
                         },
                          error: function (xhr, ajaxOptions, thrownError) {
                          
                             waitingDialog.hide();
                             $('#ajax_instr_loader').hide();   
                             //alert(xhr.status);
                             //alert(thrownError);
                             //waitingDialog.hide();
                         },
                          complete: function() {
                                             
                            waitingDialog.hide();
                            $('#ajax_instr_loader').hide();
                             
                        }

                     });
                   
                   
                }
                else{
                      $(&quot;#&quot;+lastIndex+&quot;_error&quot;).show();   
                      $(&quot;#&quot;+lastIndex+&quot;_ok&quot;).hide();  
                      disabled.attr('disabled','disabled');
                }

                return false;
            });
            $('#chkAddCustomMsg').click(function () {
                $('#dvAddCustomMsg').toggle();
            });
            $('#chkOverrideRegstrTerms').click(function () {
                $('#dvOverrideRegstrTerms').toggle();
            });
            $('#question_type').change(function () {
                $('#include_other').hide();
                $('#maximum_selections').hide();
                $('#scalefromto').hide();
                $('#dv_answer_length').hide();
                $('.que_optiopns').hide();
                $('#dv_que_options').hide();
                $('#btn_add_opt').hide();

                $('#opt_grid').hide();
                $('#otp_text').hide();
                $('#btn_add_opt').hide();

                if ($(this).find(&quot;:selected&quot;).val() == 'MultichoiceOne') {
                    $('#include_other').show();
                    $('#dv_answer_length').show();
                    $('#dv_que_options').show();
                    $('#dv_multichoice').show();

                    $('#opt_grid').show();
                    $('#otp_text').hide();
                    $('#btn_add_opt').show();
                    SetOptTextbox();

                } else if ($(this).find(&quot;:selected&quot;).val() == 'MultichoiceMany') {
                    $('#include_other').show();
                    $('#maximum_selections').show();
                    $('#dv_answer_length').show();
                    $('#dv_que_options').show();
                    $('#dv_multichoice').show();
                    $('#opt_grid').show();
                    $('#otp_text').hide();
                    $('#btn_add_opt').show();
                    SetOptTextbox();
                } else if ($(this).find(&quot;:selected&quot;).val() == 'TrueFalse') {
                    $('#dv_true_false').show();
                    $('#dv_que_options').show();
                } else if ($(this).find(&quot;:selected&quot;).val() == 'YesNo') {
                    $('#dv_yes_no').show();
                    $('#dv_que_options').show();
                } else if ($(this).find(&quot;:selected&quot;).val() == 'Scale') {
                    $('#scalefromto').show();
                }
            });
            $('#btn_add_opt').click(function () {
                SetOptTextbox();
            
                $('#opt_grid').hide();
                $('#otp_text').show();
                $('#btn_add_opt').hide();
            });
            $('#btn_add_opt_1').click(function () {
                $('#opt_grid').show();
                $('#otp_text').hide();
                $('#btn_add_opt').show();
            });
            $('#btn_cancel_opt_1').click(function () {
                $('#opt_grid').show();
                $('#otp_text').hide();
                $('#btn_add_opt').show();
            });
            $('.flate-btns').click(function () {
                $('.flate-btns').removeClass('selected');
                //$('.flate-btns').removeClass('btn-info');
                //$('.flate-btns').addClass('btn-default');
                $(this).addClass('selected');
                //$(this).addClass('btn-info');
                //$(this).removeClass('btn-default');
            });


            $('#answer_length').change(function () {
                SetOptTextbox();
            });


            $('.popup-with-form').magnificPopup({
                type: 'inline',
                preloader: false,
                focus: '#name',
                closeOnBgClick:false,
                enableEscapeKey:false
            });
            
            $(&quot;#add-manage-question-Questions&quot;).click(function(){
                
                $(&quot;#question_action&quot;).val(&quot;add&quot;);
                $(&quot;#followup_or_survey&quot;).val(&quot;survey&quot;);
                tinymce.execCommand('mceFocus',true,'txtQuestion');
                tinyMCE.activeEditor.setContent('');

                resetAddQuestionForm()
                
            })
            $(&quot;#add-manage-question-Questions-Followup&quot;).click(function(){
                
                $(&quot;#question_action&quot;).val(&quot;add&quot;);
                $(&quot;#followup_or_survey&quot;).val(&quot;followup&quot;);
                tinymce.execCommand('mceFocus',true,'txtQuestion');
                tinyMCE.activeEditor.setContent('');

                resetAddQuestionForm()
                
            })
            $('.popup-with-form-questions').magnificPopup({
                type: 'inline',
                preloader: false,
                focus: '#name',
                closeOnBgClick:false,
                enableEscapeKey:false,
                callbacks:  {
                open: function() {
                  
                  
                  tinymce.init({
                                        selector: '#txtQuestion',
                                        setup: function (editor) {
                                                        editor.on('change', function () {
                                                            editor.save();
                                                        });
                                                    },
                                        height: 150,
                                        width:650,
                                        menubar: false,
                                        plugins: [
                                          'advlist autolink lists charmap print preview anchor',
                                          'searchreplace visualblocks code fullscreen',
                                          'insertdatetime  table contextmenu paste code',
                                          'textcolor','link'
                                        ],
                                        toolbar: 'forecolor backcolor | fontselect fontsizeselect | undo redo | insert | styleselect | bold italic  | alignleft aligncenter alignright alignjustify | bullist numlist outdent indent link',
                                        fontsize_formats: &quot;8pt 10pt 12pt 14pt 18pt 24pt 36pt&quot;,
                                        textcolor_cols: &quot;5&quot;

                                      });
                                      
                                      
                            tinymce.execCommand('mceFocus',true,'txtQuestion');
                  
                  },
                 close: function(){
                     
                     
                     tinymce.remove(&quot;#txtQuestion&quot;);


                 } 
                  
               }
            });
            $('#li_course_end_questions').click(function () {
                $('#li_course_end_questions').addClass('active');
                $('#li_follow_up_questions').removeClass('active');
            });
            $('#li_follow_up_questions').click(function () {
                $('#li_follow_up_questions').addClass('active');
                $('#li_course_end_questions').removeClass('active');
            });
            $('#btn-yes').click(function () {
                $('#btn-yes').addClass('btn-success');
                $('#btn-yes').removeClass('btn-default');
                $('#btn-no').addClass('btn-default');
                $('#btn-no').removeClass('btn-warning');
            });
            $('#btn-no').click(function () {
                $('#btn-no').addClass('btn-warning');
                $('#btn-no').removeClass('btn-default');
                $('#btn-yes').addClass('btn-default');
                $('#btn-yes').removeClass('btn-success');
            });
        
            $('#btnYes').click(function () {
                $('#btnNo').addClass('btn-default');
                $('#btnNo').removeClass('btn-danger');
                $('#btnYes').removeClass('btn-default');
                $('#btnYes').addClass('btn-success');
                changeValAndLable('1');
            });
            $('#btnNo').click(function () {
                $('#btnNo').addClass('btn-danger');
                $('#btnNo').removeClass('btn-default');
                $('#btnYes').addClass('btn-default');
                $('#btnYes').removeClass('btn-success');
                changeValAndLable('0')
            });

            $('.toggle-button').click(function () {
                if ($(this).children('.i-check').hasClass('fa-check')) {
                    $(this).children('.i-check').removeClass('fa-check');
                    $(this).children('.i-check').addClass('fa-times');
                } else {
                    $(this).children('.i-check').removeClass('fa-times');
                    $(this).children('.i-check').addClass('fa-check');
                }
            });
            $('#cboCourseLocation').change(function () {
                if ($('option:selected', this).text() == 'Other') {
                    
                    $(&quot;#cboCourseLocRoom&quot;).rules(&quot;remove&quot;, &quot;required&quot;);
                    $(&quot;#txtAdhocLocationName&quot;).rules(&quot;add&quot;, &quot;required&quot;);
                    $(&quot;#txtAdhocRoomName&quot;).rules(&quot;add&quot;, &quot;required&quot;);
                    
                    $('#dv_add_location').show();
                    $('#li_room').hide();
                    
                } else {
                    
                    
                    $(&quot;#cboCourseLocRoom&quot;).rules(&quot;add&quot;, &quot;required&quot;);
                    $(&quot;#txtAdhocLocationName&quot;).rules(&quot;remove&quot;, &quot;required&quot;);
                    $(&quot;#txtAdhocRoomName&quot;).rules(&quot;remove&quot;, &quot;required&quot;);
                    
                    $('#dv_add_location').hide();
                    $('#li_room').show();
                    
                    $('#cboCourseLocRoom').empty();
                    
                    
                     $.ajax({
                        type: &quot;GET&quot;,
                        cache: false,
                        timeout:60000,
                        url: &quot;/PGCTechnologyServices/course-manager/fillcourselocroomcbo?Loc=&quot; + $(&quot;#cboCourseLocation&quot;).val()+'&amp;selected=&amp;sid=kcbtrojn&amp;time='+new Date().getTime(),
                         beforeSend:function() {
                             waitingDialog.hide();
                             $('#ajax_instr_loader').hide();
                             waitingDialog.show('Loading....');
                          }, 
                        success: function(data) {

                             $('#cboCourseLocRoom').append(data);

                           waitingDialog.hide();
                           $('#ajax_instr_loader').hide();

                        },
                         error: function (xhr, ajaxOptions, thrownError) {

                            waitingDialog.hide();
                            $('#ajax_instr_loader').hide();
                            //alert(xhr.status);
                            //alert(thrownError);
                            //waitingDialog.hide();
                          },
                          complete: function() {

                             waitingDialog.hide();
                             $('#ajax_instr_loader').hide();
                         }

                    });
                    
                      
                    
                }
            });



            $('[data-toggle=&quot;tooltip&quot;]').tooltip();

            /*
            changeCategory(0);
            $(&quot;#Num1&quot;).val();
            $(&quot;#Num2&quot;).val();
            $(&quot;#Num3&quot;).val();
            
            $(&quot;#Num3&quot;).trigger('change');
            $('#cboCourseLocation').trigger('change');
            */
        });
        function SetOptTextbox() {
            $('#opt_short').hide();
            $('#opt_medium').hide();
            $('#opt_long').hide();
            if ($('#answer_length').find(&quot;:selected&quot;).val() == 'Short') {
                $('#opt_short').show();
            } else if ($('#answer_length').find(&quot;:selected&quot;).val() == 'Medium') {
                $('#opt_medium').show();
            } else if ($('#answer_length').find(&quot;:selected&quot;).val() == 'Long') {
                $('#opt_long').show();
            }
        }
        function SelectActiveTab(index) {
            
            if (index &lt; 7) {
                $(&quot;div.list-group>div.list-group-item&quot;).siblings('div.active').removeClass(&quot;active&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(0).addClass(&quot;active&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(0).removeClass(&quot;selected&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(0).removeClass(&quot;done&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(0).prevAll().addClass(&quot;selected&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(0).prevAll().removeClass(&quot;done&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(0).nextAll().removeClass(&quot;selected&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(0).nextAll().addClass(&quot;done&quot;);
            } else if (index &lt; 13) {
                $(&quot;div.list-group>div.list-group-item&quot;).siblings('div.active').removeClass(&quot;active&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(1).addClass(&quot;active&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(1).removeClass(&quot;selected&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(1).removeClass(&quot;done&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(1).prevAll().addClass(&quot;selected&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(1).prevAll().removeClass(&quot;done&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(1).nextAll().removeClass(&quot;selected&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(1).nextAll().addClass(&quot;done&quot;);
            } else if (index &lt; 17) {
                $(&quot;div.list-group>div.list-group-item&quot;).siblings('div.active').removeClass(&quot;active&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(2).addClass(&quot;active&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(2).removeClass(&quot;selected&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(2).removeClass(&quot;done&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(2).prevAll().addClass(&quot;selected&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(2).prevAll().removeClass(&quot;done&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(2).nextAll().removeClass(&quot;selected&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(2).nextAll().addClass(&quot;done&quot;);
            } else if (index &lt; 19) {
                $(&quot;div.list-group>div.list-group-item&quot;).siblings('div.active').removeClass(&quot;active&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(3).addClass(&quot;active&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(3).removeClass(&quot;selected&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(3).removeClass(&quot;done&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(3).prevAll().addClass(&quot;selected&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(3).prevAll().removeClass(&quot;done&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(3).nextAll().removeClass(&quot;selected&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(3).nextAll().addClass(&quot;done&quot;);
            } else if (index &lt; 25) {
                $(&quot;div.list-group>div.list-group-item&quot;).siblings('div.active').removeClass(&quot;active&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(4).addClass(&quot;active&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(4).removeClass(&quot;selected&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(4).removeClass(&quot;done&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(4).prevAll().addClass(&quot;selected&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(4).prevAll().removeClass(&quot;done&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(4).nextAll().removeClass(&quot;selected&quot;);
                $(&quot;div.list-group>div.list-group-item&quot;).eq(4).nextAll().addClass(&quot;done&quot;);
            }
            
            $('.taller-menu .list-group .list-group-item small a').find(&quot;[tabindex='&quot; + index + &quot;']&quot;).addClass('active');
        }
       
       
    
    

    function encodeImageFileAsURL(fileid,imgbase64field) {
        var filesSelected = document.getElementById(fileid).files;
        if (filesSelected.length > 0) {
            var fileToLoad = filesSelected[0];
            var filetype = fileToLoad.type.toLowerCase();

            if (filetype == 'image/jpeg' || filetype == 'image/png') {
                var fileReader = new FileReader();

                fileReader.onload = function (fileLoadedEvent) {
                    var srcData = fileLoadedEvent.target.result;
                    
                    output_format = '';
                    if (filetype == &quot;image/png&quot;) {
                        output_format = &quot;png&quot;;
                    }
                    else if (filetype == &quot;image/jpeg&quot;) {
                        output_format = &quot;jpeg&quot;;
                    }
                    else if (filetype == &quot;image/jpg&quot;) {
                        output_format = &quot;jpeg&quot;;
                    }
                    //i=document.getElementById(fileid+'Hidden');
                    

                    var i = new Image();
                    //i.src = readAsDataURL(fileToLoad);
                    i.src = srcData;
                    i.onload = function () {

                        document.getElementById(imgbase64field).value = jic.compress(i, 80, output_format).src;
                        return true;
                    }

                }
                fileReader.readAsDataURL(fileToLoad);
                return true;

            }
            else {

                //alert('Please only upload png or jpeg image only');
                alert(&quot;Please only upload png or jpeg image only&quot;);
                return false;

            }
            
        }
    }

        
    $(&quot;#browse&quot;).click(function () {
        $(&quot;#test&quot;).click();
    })

    var loadFile = function (event) {

        var returndata = encodeImageFileAsURL('test',&quot;course_img&quot;);
        if (returndata) {

            var output = document.getElementById('output');
            output.src = URL.createObjectURL(event.target.files[0]);
        }


    };
    
    $(&quot;#browse_insp_img&quot;).click(function () {
        $(&quot;#test_insp&quot;).click();
    })

    var loadFile = function (event) {

        event.preventDefault();
        var returndata = encodeImageFileAsURL('test',&quot;course_img&quot;);
        if (returndata) {

            var output = document.getElementById('output');
            output.src = URL.createObjectURL(event.target.files[0]);
        }


    };
    
    var loadFile2  = function (event) {
        
        event.preventDefault();
        var returndata = encodeImageFileAsURL('browse_insp_img',&quot;test_insp_base64&quot;);
        if (returndata) {

            var output = document.getElementById('ins_profile_pic');
            output.src = URL.createObjectURL(event.target.files[0]);
        }


    };

  $(document).ajaxSend(function (event, xhr, options) {
    
    //waitingDialog.hide();
   // $('#ajax_instr_loader').hide();
    
    if(options.url.indexOf('course-manager/getinstructor') !== -1){
        
         $('#ajax_instr_loader').show(); 
    }
    else if(options.url.indexOf('course-manager/getclassgroups') !== -1){
        
         $('#ajax_instr_loader').show(); 
    }
    else{
            
            //waitingDialog.show('Loading....');
        }     
 });

$(document).ajaxComplete(function (event, xhr, options) {

    
    waitingDialog.hide();
    $('#ajax_instr_loader').hide();
    
    $('.sidebar_').height($('.main_content').height());
    
    
    if(options.url.indexOf('course-manager/getinstructor') !== -1){
        
         setTimeout(function(){ $(&quot;#txtInstructor&quot;).focus(); }, 800);

    }
    
    if(options.url.indexOf('course-manager/getclassgroups') !== -1){
        
         setTimeout(function(){ $(&quot;#txtClassEnrollGroupName&quot;).focus(); }, 800);

    }


});

$(document).ajaxSuccess(function () {

    waitingDialog.hide();
    $('#ajax_instr_loader').hide();
    $('.sidebar_').height($('.main_content').height());

});
$(document).ajaxError(function () {


    waitingDialog.hide();
    $('#ajax_instr_loader').hide();
    $('.sidebar_').height($('.main_content').height());

});
$(document).ajaxComplete (function () {


setTimeout(function(){ 

    waitingDialog.hide();
        
    $('#ajax_instr_loader').hide();
    $('.sidebar_').height($('.main_content').height());
    
}, 10);
  

});

function showErrosIfAnySingle() {
     
    //waitingDialog.show('Loading....');
    setTimeout(function(){ 
     

         lastIndex=$('.bhoechie-tab-menu.taller-menu div.list-group div.list-group-item.text-center.active small a.active').attr('tabindex');

          var finddiv=$(&quot;div&quot;).find(&quot;[data-tabindex='&quot; + lastIndex + &quot;']&quot;);

        //    var form=$(&quot;div.bhoechie-tab>div.bhoechie-tab-content&quot;).eq(lastIndex).find('form');
          var form= $(finddiv).find('form');
          tab=$(form).parent().attr('data-tabindex');
          if ($(form).valid()) {

              $(&quot;#&quot;+tab+&quot;_error&quot;).hide(); 
              $(&quot;#&quot;+tab+&quot;_ok&quot;).show();  
          }
          else{

               $(&quot;#&quot;+tab+&quot;_error&quot;).show(); 
               $(&quot;#&quot;+tab+&quot;_ok&quot;).hide();  

            }

             waitingDialog.hide();
          

     //waitingDialog.hide();
  }, 100);  

}

    
   
$(window).load(function() {
        waitingDialog.hide();
        $('.tooltip_').tooltip();
});   

window.onerror = function myErrorHandler(errorMsg, url, lineNumber) {
    
    waitingDialog.hide();
    $('#ajax_instr_loader').hide();
    return false;
}


        
        
    window.onbeforeunload = function (e) {
       
       if(finish_success==false){
           
       
        waitingDialog.show();
        //e.preventDefault();
        var tabindex=$(&quot;.bhoechie-tab-container&quot;).find('div.bhoechie-tab-content.active').attr('data-tabindex');
       
        var finddiv=$(&quot;div&quot;).find(&quot;[data-tabindex='&quot; + tabindex + &quot;']&quot;);
            //var datastring = $(&quot;div.bhoechie-tab>div.bhoechie-tab-content&quot;).eq(tabindex).find('form').serialize();
            var datastring = $(finddiv).find('form').serialize();
             $.ajax({
                 type: &quot;POST&quot;,
                 cache: false,
                 async: false,
                 url: &quot;/PGCTechnologyServices/course-manager/savedatatosession?sid=kcbtrojn&quot;,
                 data: datastring,
                 dataType: &quot;json&quot;,
                 success: function(data) {

                     //waitingDialog.hide();
                     //var obj = jQuery.parseJSON(data); if the dataType is not specified as json uncomment this
                     // do what ever you want with the server response
                 },
                  error: function (xhr, ajaxOptions, thrownError) {
                     alert(xhr.status);
                     alert(thrownError);
                     //waitingDialog.hide();
                   }

             });

        
        $.ajax({
                type: &quot;POST&quot;,
                cache: false,
                async: false,
                url: &quot;/PGCTechnologyServices/course-manager/finishddcourse?sid=kcbtrojn&quot;,
                success: function(data) {
               //   alert('hi'); 
                },
                 error: function (xhr, ajaxOptions, thrownError) {
                    //alert(xhr.status);
                    //alert(thrownError);
                    //waitingDialog.hide();
                  }

            });
            
          

      }       
        
        return undefined;
    };
 


                                 
                                
                            
                            
                            
                                Ascriptica 8.5.1 
                                © Copyright 2020 PGC Technology Services • All rights reserved
                            
                        
                        
                            
                    Loading....


                    

                        $(function () {
                            $(&quot;#datepicker&quot;).datepicker({
                                todayHighlight: false,
                                changeYear: true,
                                changeMonth: true,
                                showButtonPanel: true,
                                yearRange: &quot;-100:+100&quot;,
                                defaultDate: ''
                            });
                            $(&quot;#datepicker2&quot;).datepicker({
                                todayHighlight: false,
                                changeYear: true,
                                changeMonth: true,
                                showButtonPanel: true,
                                yearRange: &quot;-100:+100&quot;,
                                defaultDate: ''
                            });
                        });


                        $('body').on('click', '.close_notification', function (e)
                        {
                            $(&quot;.notification_area&quot;).remove();
                            return false;
                        });


                    
                    
                        $(document).ready(function () {
                            $('.main-container').css('minHeight',$(window).height());
                            $('.sidebar_').height($('.main_content').height());
                        });
                        
                        $( document ).ajaxStop(function() {
                           $('.main-container').css('minHeight',$(window).height());
                          });
                    
                 

  
                
   
        
            
                Online Users
            
            
                
            
        
    
  

     
          
             $('.popup-with-form').magnificPopup({
                type: 'inline',
                preloader: false,
                focus: '#name',
                closeOnBgClick:false,
                enableEscapeKey:false
            });


            
    
        $('body').on('click', '#add_usr_modal', function (e) {
            
             e.preventDefault();
             $('#online_users_body').load('/PGCTechnologyServices/users/online');
             $(&quot;#onlineusers_&quot;).show();
        });

  
        
    

         
  $('#badge_title').tooltip();       
  $('.tooltip__').tooltip();     
  
  $('body').on('click', '#btn_refresh', function() {
    $('.notification_area').remove();
});

  

    


JulyJanuaryFebruaryMarchAprilMayJuneJulyAugustSeptemberOctoberNovemberDecember202019501951195219531954195519561957195819591960196119621963196419651966196719681969197019711972197319741975197619771978197919801981198219831984198519861987198819891990199119921993199419951996199719981999200020012002200320042005200620072008200920102011201220132014201520162017201820192020202120222023202420252026202720282029203020312032203320342035203620372038203920402041204220432044204520462047204820492050SunMonTueWedThuFriSat282930123456789101112131415161718192021222324252627282930311Save Selected12:00 AM12:10 AM12:20 AM12:30 AM12:40 AM12:50 AM01:00 AM01:10 AM01:20 AM01:30 AM01:40 AM01:50 AM02:00 AM02:10 AM02:20 AM02:30 AM02:40 AM02:50 AM03:00 AM03:10 AM03:20 AM03:30 AM03:40 AM03:50 AM04:00 AM04:10 AM04:20 AM04:30 AM04:40 AM04:50 AM05:00 AM05:10 AM05:20 AM05:30 AM05:40 AM05:50 AM06:00 AM06:10 AM06:20 AM06:30 AM06:40 AM06:50 AM07:00 AM07:10 AM07:20 AM07:30 AM07:40 AM07:50 AM08:00 AM08:10 AM08:20 AM08:30 AM08:40 AM08:50 AM09:00 AM09:10 AM09:20 AM09:30 AM09:40 AM09:50 AM10:00 AM10:10 AM10:20 AM10:30 AM10:40 AM10:50 AM11:00 AM11:10 AM11:20 AM11:30 AM11:40 AM11:50 AM12:00 PM12:10 PM12:20 PM12:30 PM12:40 PM12:50 PM01:00 PM01:10 PM01:20 PM01:30 PM01:40 PM01:50 PM02:00 PM02:10 PM02:20 PM02:30 PM02:40 PM02:50 PM03:00 PM03:10 PM03:20 PM03:30 PM03:40 PM03:50 PM04:00 PM04:10 PM04:20 PM04:30 PM04:40 PM04:50 PM05:00 PM05:10 PM05:20 PM05:30 PM05:40 PM05:50 PM06:00 PM06:10 PM06:20 PM06:30 PM06:40 PM06:50 PM07:00 PM07:10 PM07:20 PM07:30 PM07:40 PM07:50 PM08:00 PM08:10 PM08:20 PM08:30 PM08:40 PM08:50 PM09:00 PM09:10 PM09:20 PM09:30 PM09:40 PM09:50 PM10:00 PM10:10 PM10:20 PM10:30 PM10:40 PM10:50 PM11:00 PM11:10 PM11:20 PM11:30 PM11:40 PM11:50 PMJulyJanuaryFebruaryMarchAprilMayJuneJulyAugustSeptemberOctoberNovemberDecember202019501951195219531954195519561957195819591960196119621963196419651966196719681969197019711972197319741975197619771978197919801981198219831984198519861987198819891990199119921993199419951996199719981999200020012002200320042005200620072008200920102011201220132014201520162017201820192020202120222023202420252026202720282029203020312032203320342035203620372038203920402041204220432044204520462047204820492050SunMonTueWedThuFriSat282930123456789101112131415161718192021222324252627282930311Save Selected12:00 AM12:10 AM12:20 AM12:30 AM12:40 AM12:50 AM01:00 AM01:10 AM01:20 AM01:30 AM01:40 AM01:50 AM02:00 AM02:10 AM02:20 AM02:30 AM02:40 AM02:50 AM03:00 AM03:10 AM03:20 AM03:30 AM03:40 AM03:50 AM04:00 AM04:10 AM04:20 AM04:30 AM04:40 AM04:50 AM05:00 AM05:10 AM05:20 AM05:30 AM05:40 AM05:50 AM06:00 AM06:10 AM06:20 AM06:30 AM06:40 AM06:50 AM07:00 AM07:10 AM07:20 AM07:30 AM07:40 AM07:50 AM08:00 AM08:10 AM08:20 AM08:30 AM08:40 AM08:50 AM09:00 AM09:10 AM09:20 AM09:30 AM09:40 AM09:50 AM10:00 AM10:10 AM10:20 AM10:30 AM10:40 AM10:50 AM11:00 AM11:10 AM11:20 AM11:30 AM11:40 AM11:50 AM12:00 PM12:10 PM12:20 PM12:30 PM12:40 PM12:50 PM01:00 PM01:10 PM01:20 PM01:30 PM01:40 PM01:50 PM02:00 PM02:10 PM02:20 PM02:30 PM02:40 PM02:50 PM03:00 PM03:10 PM03:20 PM03:30 PM03:40 PM03:50 PM04:00 PM04:10 PM04:20 PM04:30 PM04:40 PM04:50 PM05:00 PM05:10 PM05:20 PM05:30 PM05:40 PM05:50 PM06:00 PM06:10 PM06:20 PM06:30 PM06:40 PM06:50 PM07:00 PM07:10 PM07:20 PM07:30 PM07:40 PM07:50 PM08:00 PM08:10 PM08:20 PM08:30 PM08:40 PM08:50 PM09:00 PM09:10 PM09:20 PM09:30 PM09:40 PM09:50 PM10:00 PM10:10 PM10:20 PM10:30 PM10:40 PM10:50 PM11:00 PM11:10 PM11:20 PM11:30 PM11:40 PM11:50 PMJulyJanuaryFebruaryMarchAprilMayJuneJulyAugustSeptemberOctoberNovemberDecember202019501951195219531954195519561957195819591960196119621963196419651966196719681969197019711972197319741975197619771978197919801981198219831984198519861987198819891990199119921993199419951996199719981999200020012002200320042005200620072008200920102011201220132014201520162017201820192020202120222023202420252026202720282029203020312032203320342035203620372038203920402041204220432044204520462047204820492050SunMonTueWedThuFriSat282930123456789101112131415161718192021222324252627282930311Save Selected00:0000:1000:2000:3000:4000:5001:0001:1001:2001:3001:4001:5002:0002:1002:2002:3002:4002:5003:0003:1003:2003:3003:4003:5004:0004:1004:2004:3004:4004:5005:0005:1005:2005:3005:4005:5006:0006:1006:2006:3006:4006:5007:0007:1007:2007:3007:4007:5008:0008:1008:2008:3008:4008:5009:0009:1009:2009:3009:4009:5010:0010:1010:2010:3010:4010:5011:0011:1011:2011:3011:4011:5012:0012:1012:2012:3012:4012:5013:0013:1013:2013:3013:4013:5014:0014:1014:2014:3014:4014:5015:0015:1015:2015:3015:4015:5016:0016:1016:2016:3016:4016:5017:0017:1017:2017:3017:4017:5018:0018:1018:2018:3018:4018:5019:0019:1019:2019:3019:4019:5020:0020:1020:2020:3020:4020:5021:0021:1021:2021:3021:4021:5022:0022:1022:2022:3022:4022:5023:0023:1023:2023:3023:4023:50JulyJanuaryFebruaryMarchAprilMayJuneJulyAugustSeptemberOctoberNovemberDecember202019501951195219531954195519561957195819591960196119621963196419651966196719681969197019711972197319741975197619771978197919801981198219831984198519861987198819891990199119921993199419951996199719981999200020012002200320042005200620072008200920102011201220132014201520162017201820192020202120222023202420252026202720282029203020312032203320342035203620372038203920402041204220432044204520462047204820492050SunMonTueWedThuFriSat282930123456789101112131415161718192021222324252627282930311Save Selected00:0000:1000:2000:3000:4000:5001:0001:1001:2001:3001:4001:5002:0002:1002:2002:3002:4002:5003:0003:1003:2003:3003:4003:5004:0004:1004:2004:3004:4004:5005:0005:1005:2005:3005:4005:5006:0006:1006:2006:3006:4006:5007:0007:1007:2007:3007:4007:5008:0008:1008:2008:3008:4008:5009:0009:1009:2009:3009:4009:5010:0010:1010:2010:3010:4010:5011:0011:1011:2011:3011:4011:5012:0012:1012:2012:3012:4012:5013:0013:1013:2013:3013:4013:5014:0014:1014:2014:3014:4014:5015:0015:1015:2015:3015:4015:5016:0016:1016:2016:3016:4016:5017:0017:1017:2017:3017:4017:5018:0018:1018:2018:3018:4018:5019:0019:1019:2019:3019:4019:5020:0020:1020:2020:3020:4020:5021:0021:1021:2021:3021:4021:5022:0022:1022:2022:3022:4022:5023:0023:1023:2023:3023:4023:50JulyJanuaryFebruaryMarchAprilMayJuneJulyAugustSeptemberOctoberNovemberDecember202019501951195219531954195519561957195819591960196119621963196419651966196719681969197019711972197319741975197619771978197919801981198219831984198519861987198819891990199119921993199419951996199719981999200020012002200320042005200620072008200920102011201220132014201520162017201820192020202120222023202420252026202720282029203020312032203320342035203620372038203920402041204220432044204520462047204820492050SunMonTueWedThuFriSat282930123456789101112131415161718192021222324252627282930311Save Selected00:0000:1000:2000:3000:4000:5001:0001:1001:2001:3001:4001:5002:0002:1002:2002:3002:4002:5003:0003:1003:2003:3003:4003:5004:0004:1004:2004:3004:4004:5005:0005:1005:2005:3005:4005:5006:0006:1006:2006:3006:4006:5007:0007:1007:2007:3007:4007:5008:0008:1008:2008:3008:4008:5009:0009:1009:2009:3009:4009:5010:0010:1010:2010:3010:4010:5011:0011:1011:2011:3011:4011:5012:0012:1012:2012:3012:4012:5013:0013:1013:2013:3013:4013:5014:0014:1014:2014:3014:4014:5015:0015:1015:2015:3015:4015:5016:0016:1016:2016:3016:4016:5017:0017:1017:2017:3017:4017:5018:0018:1018:2018:3018:4018:5019:0019:1019:2019:3019:4019:5020:0020:1020:2020:3020:4020:5021:0021:1021:2021:3021:4021:5022:0022:1022:2022:3022:4022:5023:0023:1023:2023:3023:4023:50JulyJanuaryFebruaryMarchAprilMayJuneJulyAugustSeptemberOctoberNovemberDecember202019501951195219531954195519561957195819591960196119621963196419651966196719681969197019711972197319741975197619771978197919801981198219831984198519861987198819891990199119921993199419951996199719981999200020012002200320042005200620072008200920102011201220132014201520162017201820192020202120222023202420252026202720282029203020312032203320342035203620372038203920402041204220432044204520462047204820492050SunMonTueWedThuFriSat282930123456789101112131415161718192021222324252627282930311Save Selected00:0000:1000:2000:3000:4000:5001:0001:1001:2001:3001:4001:5002:0002:1002:2002:3002:4002:5003:0003:1003:2003:3003:4003:5004:0004:1004:2004:3004:4004:5005:0005:1005:2005:3005:4005:5006:0006:1006:2006:3006:4006:5007:0007:1007:2007:3007:4007:5008:0008:1008:2008:3008:4008:5009:0009:1009:2009:3009:4009:5010:0010:1010:2010:3010:4010:5011:0011:1011:2011:3011:4011:5012:0012:1012:2012:3012:4012:5013:0013:1013:2013:3013:4013:5014:0014:1014:2014:3014:4014:5015:0015:1015:2015:3015:4015:5016:0016:1016:2016:3016:4016:5017:0017:1017:2017:3017:4017:5018:0018:1018:2018:3018:4018:5019:0019:1019:2019:3019:4019:5020:0020:1020:2020:3020:4020:5021:0021:1021:2021:3021:4021:5022:0022:1022:2022:3022:4022:5023:0023:1023:2023:3023:4023:50JulyJanuaryFebruaryMarchAprilMayJuneJulyAugustSeptemberOctoberNovemberDecember202019501951195219531954195519561957195819591960196119621963196419651966196719681969197019711972197319741975197619771978197919801981198219831984198519861987198819891990199119921993199419951996199719981999200020012002200320042005200620072008200920102011201220132014201520162017201820192020202120222023202420252026202720282029203020312032203320342035203620372038203920402041204220432044204520462047204820492050SunMonTueWedThuFriSat282930123456789101112131415161718192021222324252627282930311Save Selected00:0000:1000:2000:3000:4000:5001:0001:1001:2001:3001:4001:5002:0002:1002:2002:3002:4002:5003:0003:1003:2003:3003:4003:5004:0004:1004:2004:3004:4004:5005:0005:1005:2005:3005:4005:5006:0006:1006:2006:3006:4006:5007:0007:1007:2007:3007:4007:5008:0008:1008:2008:3008:4008:5009:0009:1009:2009:3009:4009:5010:0010:1010:2010:3010:4010:5011:0011:1011:2011:3011:4011:5012:0012:1012:2012:3012:4012:5013:0013:1013:2013:3013:4013:5014:0014:1014:2014:3014:4014:5015:0015:1015:2015:3015:4015:5016:0016:1016:2016:3016:4016:5017:0017:1017:2017:3017:4017:5018:0018:1018:2018:3018:4018:5019:0019:1019:2019:3019:4019:5020:0020:1020:2020:3020:4020:5021:0021:1021:2021:3021:4021:5022:0022:1022:2022:3022:4022:5023:0023:1023:2023:3023:4023:50JulyJanuaryFebruaryMarchAprilMayJuneJulyAugustSeptemberOctoberNovemberDecember202019501951195219531954195519561957195819591960196119621963196419651966196719681969197019711972197319741975197619771978197919801981198219831984198519861987198819891990199119921993199419951996199719981999200020012002200320042005200620072008200920102011201220132014201520162017201820192020202120222023202420252026202720282029203020312032203320342035203620372038203920402041204220432044204520462047204820492050SunMonTueWedThuFriSat282930123456789101112131415161718192021222324252627282930311Save Selected00:0001:0002:0003:0004:0005:0006:0007:0008:0009:0010:0011:0012:0013:0014:0015:0016:0017:0018:0019:0020:0021:0022:0023:00JulyJanuaryFebruaryMarchAprilMayJuneJulyAugustSeptemberOctoberNovemberDecember202019501951195219531954195519561957195819591960196119621963196419651966196719681969197019711972197319741975197619771978197919801981198219831984198519861987198819891990199119921993199419951996199719981999200020012002200320042005200620072008200920102011201220132014201520162017201820192020202120222023202420252026202720282029203020312032203320342035203620372038203920402041204220432044204520462047204820492050SunMonTueWedThuFriSat282930123456789101112131415161718192021222324252627282930311Save Selected00:0000:1000:2000:3000:4000:5001:0001:1001:2001:3001:4001:5002:0002:1002:2002:3002:4002:5003:0003:1003:2003:3003:4003:5004:0004:1004:2004:3004:4004:5005:0005:1005:2005:3005:4005:5006:0006:1006:2006:3006:4006:5007:0007:1007:2007:3007:4007:5008:0008:1008:2008:3008:4008:5009:0009:1009:2009:3009:4009:5010:0010:1010:2010:3010:4010:5011:0011:1011:2011:3011:4011:5012:0012:1012:2012:3012:4012:5013:0013:1013:2013:3013:4013:5014:0014:1014:2014:3014:4014:5015:0015:1015:2015:3015:4015:5016:0016:1016:2016:3016:4016:5017:0017:1017:2017:3017:4017:5018:0018:1018:2018:3018:4018:5019:0019:1019:2019:3019:4019:5020:0020:1020:2020:3020:4020:5021:0021:1021:2021:3021:4021:5022:0022:1022:2022:3022:4022:5023:0023:1023:2023:3023:4023:50JulyJanuaryFebruaryMarchAprilMayJuneJulyAugustSeptemberOctoberNovemberDecember202019501951195219531954195519561957195819591960196119621963196419651966196719681969197019711972197319741975197619771978197919801981198219831984198519861987198819891990199119921993199419951996199719981999200020012002200320042005200620072008200920102011201220132014201520162017201820192020202120222023202420252026202720282029203020312032203320342035203620372038203920402041204220432044204520462047204820492050SunMonTueWedThuFriSat282930123456789101112131415161718192021222324252627282930311Save Selected00:0000:1000:2000:3000:4000:5001:0001:1001:2001:3001:4001:5002:0002:1002:2002:3002:4002:5003:0003:1003:2003:3003:4003:5004:0004:1004:2004:3004:4004:5005:0005:1005:2005:3005:4005:5006:0006:1006:2006:3006:4006:5007:0007:1007:2007:3007:4007:5008:0008:1008:2008:3008:4008:5009:0009:1009:2009:3009:4009:5010:0010:1010:2010:3010:4010:5011:0011:1011:2011:3011:4011:5012:0012:1012:2012:3012:4012:5013:0013:1013:2013:3013:4013:5014:0014:1014:2014:3014:4014:5015:0015:1015:2015:3015:4015:5016:0016:1016:2016:3016:4016:5017:0017:1017:2017:3017:4017:5018:0018:1018:2018:3018:4018:5019:0019:1019:2019:3019:4019:5020:0020:1020:2020:3020:4020:5021:0021:1021:2021:3021:4021:5022:0022:1022:2022:3022:4022:5023:0023:1023:2023:3023:4023:50JulyJanuaryFebruaryMarchAprilMayJuneJulyAugustSeptemberOctoberNovemberDecember202019501951195219531954195519561957195819591960196119621963196419651966196719681969197019711972197319741975197619771978197919801981198219831984198519861987198819891990199119921993199419951996199719981999200020012002200320042005200620072008200920102011201220132014201520162017201820192020202120222023202420252026202720282029203020312032203320342035203620372038203920402041204220432044204520462047204820492050SunMonTueWedThuFriSat282930123456789101112131415161718192021222324252627282930311Save Selected12:00 AM12:10 AM12:20 AM12:30 AM12:40 AM12:50 AM01:00 AM01:10 AM01:20 AM01:30 AM01:40 AM01:50 AM02:00 AM02:10 AM02:20 AM02:30 AM02:40 AM02:50 AM03:00 AM03:10 AM03:20 AM03:30 AM03:40 AM03:50 AM04:00 AM04:10 AM04:20 AM04:30 AM04:40 AM04:50 AM05:00 AM05:10 AM05:20 AM05:30 AM05:40 AM05:50 AM06:00 AM06:10 AM06:20 AM06:30 AM06:40 AM06:50 AM07:00 AM07:10 AM07:20 AM07:30 AM07:40 AM07:50 AM08:00 AM08:10 AM08:20 AM08:30 AM08:40 AM08:50 AM09:00 AM09:10 AM09:20 AM09:30 AM09:40 AM09:50 AM10:00 AM10:10 AM10:20 AM10:30 AM10:40 AM10:50 AM11:00 AM11:10 AM11:20 AM11:30 AM11:40 AM11:50 AM12:00 PM12:10 PM12:20 PM12:30 PM12:40 PM12:50 PM01:00 PM01:10 PM01:20 PM01:30 PM01:40 PM01:50 PM02:00 PM02:10 PM02:20 PM02:30 PM02:40 PM02:50 PM03:00 PM03:10 PM03:20 PM03:30 PM03:40 PM03:50 PM04:00 PM04:10 PM04:20 PM04:30 PM04:40 PM04:50 PM05:00 PM05:10 PM05:20 PM05:30 PM05:40 PM05:50 PM06:00 PM06:10 PM06:20 PM06:30 PM06:40 PM06:50 PM07:00 PM07:10 PM07:20 PM07:30 PM07:40 PM07:50 PM08:00 PM08:10 PM08:20 PM08:30 PM08:40 PM08:50 PM09:00 PM09:10 PM09:20 PM09:30 PM09:40 PM09:50 PM10:00 PM10:10 PM10:20 PM10:30 PM10:40 PM10:50 PM11:00 PM11:10 PM11:20 PM11:30 PM11:40 PM11:50 PMJulyJanuaryFebruaryMarchAprilMayJuneJulyAugustSeptemberOctoberNovemberDecember202019501951195219531954195519561957195819591960196119621963196419651966196719681969197019711972197319741975197619771978197919801981198219831984198519861987198819891990199119921993199419951996199719981999200020012002200320042005200620072008200920102011201220132014201520162017201820192020202120222023202420252026202720282029203020312032203320342035203620372038203920402041204220432044204520462047204820492050SunMonTueWedThuFriSat282930123456789101112131415161718192021222324252627282930311Save Selected12:00 AM12:10 AM12:20 AM12:30 AM12:40 AM12:50 AM01:00 AM01:10 AM01:20 AM01:30 AM01:40 AM01:50 AM02:00 AM02:10 AM02:20 AM02:30 AM02:40 AM02:50 AM03:00 AM03:10 AM03:20 AM03:30 AM03:40 AM03:50 AM04:00 AM04:10 AM04:20 AM04:30 AM04:40 AM04:50 AM05:00 AM05:10 AM05:20 AM05:30 AM05:40 AM05:50 AM06:00 AM06:10 AM06:20 AM06:30 AM06:40 AM06:50 AM07:00 AM07:10 AM07:20 AM07:30 AM07:40 AM07:50 AM08:00 AM08:10 AM08:20 AM08:30 AM08:40 AM08:50 AM09:00 AM09:10 AM09:20 AM09:30 AM09:40 AM09:50 AM10:00 AM10:10 AM10:20 AM10:30 AM10:40 AM10:50 AM11:00 AM11:10 AM11:20 AM11:30 AM11:40 AM11:50 AM12:00 PM12:10 PM12:20 PM12:30 PM12:40 PM12:50 PM01:00 PM01:10 PM01:20 PM01:30 PM01:40 PM01:50 PM02:00 PM02:10 PM02:20 PM02:30 PM02:40 PM02:50 PM03:00 PM03:10 PM03:20 PM03:30 PM03:40 PM03:50 PM04:00 PM04:10 PM04:20 PM04:30 PM04:40 PM04:50 PM05:00 PM05:10 PM05:20 PM05:30 PM05:40 PM05:50 PM06:00 PM06:10 PM06:20 PM06:30 PM06:40 PM06:50 PM07:00 PM07:10 PM07:20 PM07:30 PM07:40 PM07:50 PM08:00 PM08:10 PM08:20 PM08:30 PM08:40 PM08:50 PM09:00 PM09:10 PM09:20 PM09:30 PM09:40 PM09:50 PM10:00 PM10:10 PM10:20 PM10:30 PM10:40 PM10:50 PM11:00 PM11:10 PM11:20 PM11:30 PM11:40 PM11:50 PMid(&quot;pleaseWaitDialog&quot;)/div[@class=&quot;modal-backdrop  in&quot;]</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;js flexbox canvas canvastext webgl no-touch geolocation postmessage websqldatabase indexeddb hashchange history draganddrop websockets rgba hsla multiplebgs backgroundsize borderimage borderradius boxshadow textshadow opacity cssanimations csscolumns cssgradients cssreflections csstransforms csstransforms3d csstransitions fontface generatedcontent video audio localstorage sessionstorage webworkers no-applicationcache svg inlinesvg smil svgclippaths&quot;]/body[@class=&quot;modal-open&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
   </webElementXpaths>
</WebElementEntity>
