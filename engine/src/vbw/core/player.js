/* 
*  Player basic component 
*  @auth [ Fuu ]
*  @creator Fuu
*  @date 2025-04-23
*  @functions
*  1.save the location.
*  2.save the body parameters.
*/

import Toolbox from "../lib/toolbox";

const reg={
    name:"player",       //组件名称
    category:'system',      //组件分类
}

//配置数据，初始化可以放在这里
const config={
    location:{
        block:[1987,519],
        world:0,
        position:[8,8,0],
        rotation:[Math.PI*0.5,0,0],
    },
    body:{
        height:1.5,		//默认的人物高度
        shoulder:0.5,	//player的肩宽
        chest:0.22,		//player的胸厚
    },
    capacity:{
        move:0.03,          //每次移动的距离m，测试时候用的0.03
        rotate:0.05,        //头部旋转的的速度
        span:0.31,          //走过的高差
        squat:0.1,          //蹲下的高度
        jump:1,	            //跳过的高差
        death:3,            //死亡坠落高度
        speed:1.5,          //移动速度，m/s，测试时候用1.5
        strength:1,         //蓄力，用于跳跃
    },
    extend:2,               //周边显示的扩展数
    autosave:60,            //多少帧进行自动位置保存
}

//模拟数据，测试用的
const mock={
    start:{
        block:[2025,501],
        world:0,
        position:[8,4,1,7],
        rotation:[Math.PI*0.5,0,0],
        headAx:"y",
    },
}

const self={
    hooks:{
        reg:()=>{
            return reg;
        },
        init:()=>{
            const py=Toolbox.clone(config);
            py.avatar="";
            py.address="";
            py.stamp=Toolbox.stamp();
            return {
                chain:["env","player"],
                value:py,
            };
        },
    },
}

let count=0;
const vbw_player={
    hooks:self.hooks,

    //set the player parameters
    body:()=>{

    },

    autosave:()=>{
        if(count>config.autosave){
            console.log(`Player status saved.`);
            count=0;
        }else{
            count++;
        }
    },

    //get the player status.
    start:(ck)=>{
        const player=Toolbox.clone(mock.start);
        //1.获取本地的启动信息

        //2.从网络端获取player的信息
        //const chain=[];

        return ck && ck(player);
    },
}

export default vbw_player;