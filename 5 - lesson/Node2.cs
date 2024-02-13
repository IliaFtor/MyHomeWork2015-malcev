using NPOI.SS.Formula.Functions;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;


namespace lesson_5_Saod
{
    public class Node2<U>
    {
        public U Data { get; }
        public Node2<U> Next { get; set; }
        public Node2<U> Prev { get; set; }

        public Node2(U data)
        {
            Data = data;
        }
    }
}
