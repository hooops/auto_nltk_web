   #-*- coding:utf-8 -*-
import numpy as np
import hashlib

def d():
 
      f=open("../ads.1","w")
      li=[]
      h1="""
      <?xml version=\"1.0\" encoding=\"UTF-8\"?>
      <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">
      """
      for i in range(0,50000):
        
         d0=hashlib.md5(str(i).encode(encoding='UTF-8')).hexdigest()
         d1=hashlib.md5(str(d0).encode(encoding='UTF-8')).hexdigest()


         xmlbody="""<url>
         <loc>http://en0.figureaspect.com/html/"""+d1+d1+d1+""".html</loc>
         <lastmod>2024-02-16</lastmod>
         <changefreq>daily</changefreq>
         <priority>1.0</priority>
         </url>
         """
         li.append(xmlbody)
     
      f.write(h1+"".join(li)+"""</urlset>
      """)
    
d()
   