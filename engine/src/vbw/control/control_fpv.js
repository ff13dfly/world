/* 
*  VBW world entry
*  @auth [ Fuu ]
*  @creator Fuu
*  @date 2025-04-25
*  @functions
*  1. 
*/

import Movment from "../core/movement";
import VBW from "../core/framework";

const reg={
    name:"con_first",       //组件名称
    category:'controller',      //组件分类
}

let player=null;            //player的信息
let camera=null;            //视角相机
let actions=null;           //按键信息
let side=null;              //获取block的边长尺寸
let container=null;         //fpv启动的container
let world=null;             //当前活动的world
const config={
    id:"fpv_control",
    code:{          //键盘位控制预定义
        FORWARD:    87,     //W
        BACKWARD:   83,     //S
        LEFT:       65,     //A
        RIGHT:      68,     //D
        BODY_RISE:  82,     //R
        BODY_FALL:  70,     //F
        HEAD_LEFT:  37,     //Arrow left
        HEAD_RIGHT: 39,     //Arrow right
        HEAD_RISE:  38,     //Arrow up
        HEAD_DOWN:  40,     //Arrow down
        JUMP:       32,     //Space
        SQUAT:      17,     //Ctrl
    },
    queue:"keyboard",
    move:{
        distance:100,
        angle:Math.PI*0.01,
    },
}

const status={
    locked:false,               //是否锁定运动
    limit:null,                 //运动的限制
};

const todo={
    FORWARD:    Movment.body.forward,
    BACKWARD:   Movment.body.backward,
    LEFT:       Movment.body.leftward,
    RIGHT:      Movment.body.rightward,
    BODY_RISE:  Movment.body.rise,
    BODY_FALL:  Movment.body.fall,
    JUMP:       Movment.body.jump,
    SQUAT:      Movment.body.squat,
    HEAD_LEFT:  Movment.head.left,
    HEAD_RIGHT: Movment.head.right,
    HEAD_RISE:  Movment.head.up,            //头部旋转向上
    HEAD_DOWN:  Movment.head.down,
}

const self={
    hooks:{
        reg:()=>{ return reg},
    },
    unbind:(evt,fun)=>{
        document.removeEventListener(evt, fun);
    },
    bind:(evt,fun,dom_id)=>{
        if(!dom_id) document.addEventListener(evt, fun);
    },
    lock:()=>{
        status.locked=true;
    },
    recover:()=>{
        status.locked=false;
    },
    keyboard:(dom_id)=>{
        VBW.queue.init(config.queue);
        self.bind('keydown',(ev)=>{
            const code=ev.which;
            if(config.keyboard[code]) VBW.queue.insert(config.queue,config.keyboard[code]);
        });

        self.bind('keyup',(ev)=>{
            const code=ev.which;
            if(config.keyboard[code]) VBW.queue.remove(config.queue,config.keyboard[code]);
        });
    },

    screen:(dom_id)=>{

    },
    active:(world,dom_id)=>{
        if(actions===null) actions=VBW.queue.get(config.queue);
        if(camera===null) {
            const chain=["active","containers",dom_id,"camera"];
            camera=VBW.cache.get(chain);
        }
        if(player===null){
            const chain=["env","player"];
            player=VBW.cache.get(chain);
        }

        if(side===null){
            side=VBW.cache.get(["env","world","side"]);
        }
    },

    flip:(obj)=>{
        return Object.entries(obj).reduce((acc, [key, value]) => {
            acc[value] = key;
            return acc;
        }, {});
    },

    updateLocation:(px,py,moved,rotated)=>{
        //1.同步player的位置
        if(moved){
            const x=Math.floor(px/side[0]+1);
            const y=Math.floor(py/side[1]+1);
            //console.log(`Current ${JSON.stringify([x,y])}, player: ${JSON.stringify(player.location)}`)

            //2.处理跨越block的数据获取
            const [bx,by]=player.location;
            if(bx!==x || by!==y){
                console.log(`Cross block from  ${JSON.stringify(player.location)} to ${JSON.stringify([x,y])}`);

                const tasks=VBW.cache.get(["task",container,world]);
                tasks.push({adjunct:"block",act:"remove",param:{x:bx,y:by}});
                //tasks.push({adjunct:"block",act:"remove",param:{x:x+1,y:y}});
                //tasks.push({adjunct:"block",act:"remove",param:{x:x,y:y+1}});

                tasks.push({adjunct:"wall",act:"set",x:bx,y:by,world:0,param:{index:0,x:1.9}});
            }

            VBW.update(container,world);
        }
    },

    //帧检测的方法，在这里进行运动
    action:()=>{
        //console.log(JSON.stringify(actions));
        const dis=[config.move.distance,config.move.angle];
        const ak=camera.rotation.y;
        
        //1.检测键盘操作
        let moved=false,rotated=false;
        for(let i=0;i<actions.length;i++){
            const act=actions[i];
            if(!todo[act]) continue;
            const diff=todo[act](dis,ak);

            
            if(diff.position){
                moved=true;
                camera.position.set(
                    camera.position.x+diff.position[0],
                    camera.position.y+diff.position[1],
                    camera.position.z+diff.position[2],
                );
            }

            if(diff.rotation){
                rotated=true;
                camera.rotation.set(
                    camera.rotation.x+diff.rotation[0],
                    camera.rotation.y+diff.rotation[1],
                    camera.rotation.z+diff.rotation[2],
                );
            }

            //TODO,这里对判断是否为数组，如果是的话，连续运动，lock住再动。这样就可以支持jump等操作
            if(diff.group){
                self.lock();
                //{action:[],final:{position:[x,y,z],rotation:[rx,ry,rz]}};
            }
        }

        //2.检测是否移动出了block位置
        if(!status.lock){
            self.updateLocation(camera.position.x,camera.position.y,moved,rotated);
        }
    },
}

//https://o370968.ingest.sentry.io/api/6260025/envelope/?sentry_key=c92db65910024196aa808ea164cd9ba3&sentry_version=7&sentry_client=sentry.javascript.react%2F7.61.0 net::ERR_INTERNET_DISCONNECTED

const control_fpv={
    hooks:self.hooks,
    construct:()=>{
        const check=document.getElementById(config.id);
        if(check===null){
            const str=`<div id=${config.id}></div>`;
            const parser = new DOMParser();
            const doc = parser.parseFromString(str, 'text/html');
            return doc.body.firstChild
        }
    },

    start:(dom_id)=>{
        console.log(`Start to get the input from outside, bind html events.`);
        //0.设置dom_id和控制器的关联
        container=dom_id

        //1.增加键盘的操作
        self.keyboard(dom_id);

        //2.增加screen的操作;
        self.screen(dom_id);

        //3.设置帧同步处理
        world=VBW.cache.get(["active","world"]);
        const chain=["block",dom_id,world,"loop"];
        if(!VBW.cache.exsist(chain)) VBW.cache.set(chain,[]);
        const queue=VBW.cache.get(chain);
        queue.push({name:"movement",fun:self.action});

        //4.获取到对应的变量，方便操作
        self.active(world,dom_id);

        //5.将键盘操作，处理成运动
        if(config.keyboard===undefined) config.keyboard=self.flip(config.code);

        //console.log(camera);
    },
}

export default control_fpv;