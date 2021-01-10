from colorama import init, Fore, Style
#from colorama.Fore import RED as r, GREEN as g, Yellow as y 
#from coloram.Style import RESET_ALL as o

r = Fore.RED
g = Fore.GREEN
y = Fore.YELLOW
p = Fore.MAGENTA
o = Style.RESET_ALL
init()

print(f"""
{(r+"="+g+"=")*40}{o}
       *       *       *      *       *       *       *       *       *      *
   *      *        *      *       *   {y}#{o}   *      *       *       *       *
       *       *       *      *     {g}/   \\{o}     *       *        *       *
   *      *        *      *       *{g}/     \\{o}       *       *       *     
       *       *       *      *   {g}/__   __\\{o}   *       *       *       *
   *      *       *       *       * {g}/   \\{o}        *       *       *      
       *       *       *           {g}/     \\{o}    *       *       *       *
   *      *       *       *       {g}/_     _\\{o}           *        
       *       *       *       *    {g}/   \\{o}       *       *       *        *
                                   {g}/     \\{o}  *              *       *
            *          *          {g}/       \\{o}       *
                                 {g}/____ ____\\{o}
                                     {o}| |       !!!MERRY CHRISTMAS!!!
                                                                      \{p}
                                                                           # 
                                                         #    ##    ##    ###
                                                          #  #  #  #  #  #   
{(r+"="+g+"=")*40}{o}
"""
)


