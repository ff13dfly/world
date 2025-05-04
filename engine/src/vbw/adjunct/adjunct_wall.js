/* 
*  Module wall
*  @auth [ Fuu ]
*  @creator Fuu
*  @date 2025-04-24
*  @functions
*  1.single wall
*/

//import Framework from "../core/framework";
const reg={
    name:"wall",        //组件名称
    category:"adjunct",     //组件分类
    short:"a1",         //key的缩写，用于减少链上数据
    desc:"Wall with texture. Hole on it support.",
    version:"1.0.0",
}

const config={
    default:[[1.5,0.2,0.5],[1,0.3,0],[0,0,0],2,[1,1],[0],[],2025],       //2025为默认version
    hole:[0.5,0.6,0.9,0.6,2025],        //[ offset,width,height,windowsill,version ]
	definition:{
		2025:[
            ['x','y','z'],
            ['ox','oy','oz'],
            ['rx','ry','rz'],
            'texture_id',           //由链上合约管理，可以控制被版权问题封禁的
            ['rpx','rpy'],
            'animate',
            ["hole"],
        ],		//2018版本的数据结构
	},
    color:0xf8f8f8,        //材质加载失败的替换颜色
    grid:{					//辅助定位格栅的配置
        offsetX:0.5,		//相对x轴的偏移
        offsetY:0.5,		//相对y轴的偏移
    },
    animate:[               //支持的动画效果
        {way:"fadeout"},
        {way:"fadein"},
        {way:"moveup"},
    ],
}

const self={
    hooks:{
        reg:()=>{
            return reg;
        },
        task:()=>{
            console.log(`wall task here.`);
        },
        animate:(ms)=>{
            
        },
    },
    
    attribute:{
        add:()=>{

        },
        set:()=>{

        },
        del:()=>{

        },
        combine:()=>{

        },

        revise:(p,data,limit)=>{

        },
    },
    transform:{
        //链上数据转换成std的中间体
        //return [objs, preload]
        raw_std:(arr,cvt)=>{
            const rst=[]
            for(let i in arr){
                const d=arr[i],s=d[0],p=d[1],r=d[2],tid=d[3],rpt=d[4];
                const dt={
                    x:s[0]*cvt,y:s[1]*cvt,z:s[2]*cvt,
                    ox:p[0]*cvt,oy:p[1]*cvt,oz:p[2]*cvt+s[2]*cvt*0.5,
                    rx:r[0],ry:r[1],rz:r[2],
                    material:{
                        texture:tid,
                        repeat:rpt,
                        color:config.color,
                    },
                    animate:Array.isArray(d[5])?d[5]:null,
                    //stop:d[6],
                }
                rst.push(dt);
            }
            return rst;
        },

        //std中间体，转换成3D需要的object
        std_3d:(stds,va)=>{
            //console.log(`Elevation: ${va}`);
            const arr=[];
            for(let i=0;i<stds.length;i++){
                const row=stds[i];
                const three={
                    type:"box",
                    index:i,
                    params:{
                        size:[row.x,row.y,row.z],
                        position:[row.ox,row.oy,row.oz+va],
                        rotation:[row.rx,row.ry,row.rz],
                    },
                    material:row.material,
                }
                if(row.animate!==null) three.animate=row.animate;
                arr.push(three);
            }
            return arr;
        },


        //3D高亮时候，需要的3D的object
        std_active:(stds,va,index)=>{
            const ds={stop:[],helper:[]};
            return ds;
        },

        std_raw:(arr,cvt)=>{

        },

        std_box:(obj)=>{

        },

        //std中间体，转换成2D需要的数据
        std_2d:(arr,face)=>{

        },

        

        //2D高亮时候，需要的2D的object
        active_2d:()=>{

        },
    },
    
};

const adj_wall={
    hooks:self.hooks,               //注册的hook部分，供主动调用
    transform:self.transform,
    //attribute:self.attribute,
    animate:{

    },

    //数据属性处理
    

    //显示操作菜单
    menu:{

    },

    //控制响应
    control:{
        swipe:()=>{

        },
    },
}

export default adj_wall;