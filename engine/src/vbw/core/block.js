/* 
*  VBW block component
*  @auth [ Fuu ]
*  @creator Fuu
*  @date 2025-04-23
*  @functions
*  1.
*/

const reg={
    name:"block",
    category:"system",
}

const config={
    opacity:1,
    texture:1,          //土地的texture的id
    color:0xeeeee,    //不贴图时，block的颜色
    repeat:[10,10],
};

const self={
    hooks:{
        reg:()=>{
            return reg;
        },
        animate:(mesh)=>{
            console.log(mesh);
        },
    },
    attribute:{
        add:(p)=>{

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
        raw_std:(obj,cvt,side)=>{
            const va=obj[0];
            const status=obj[1];

            //FIXME,这里需要去获取到block的边长.
            //console.log(side);
            const s=side[0],hs=0.5*s;
			const data={
				x:s,y:s,z:va*cvt,
				ox:hs,oy:hs,oz:va*cvt*0.5,
                rx:0,ry:0,rz:0,
                status:status,
                material:{
                    texture:config.texture,
                    color:config.color,
                    repeat:config.repeat,
                },
			}
            return [data];
        },

        //std中间体，转换成3D需要的object
        std_3d:(bks)=>{
            const arr=[];
            for(let i=0;i<bks.length;i++){
                const row=bks[i];
                arr.push({
                    type:"box",
                    params:{
                        size:[row.x,row.y,row.z],
                        position:[row.ox,row.oy,row.oz],
                        rotation:[row.rx,row.ry,row.rz],
                    },
                    material:row.material,
                });
            }
            return arr;
        },

        std_active:(obj,cvt,side)=>{
            const ds={stop:[],helper:[]};
            console.log(obj);
            const row=obj[0];
            const arr=[];
            arr.push({
                type:"plane",
                params:{
                    size:[row.x,row.y,0],
                    position:[0,0,0],
                    rotation:[row.rx,row.ry,row.rz],
                },
                material:{
                    color:0xff0000,
                },
            })

            ds.helper=arr;
            return ds;
        },

        std_raw:(arr,cvt)=>{

        },

        std_box:(obj)=>{

        },

        

        //std中间体，转换成2D需要的数据
        std_2d:(arr,face)=>{

        },

        //3D高亮时候，需要的3D的object
        acitve_3d:()=>{

        },

        //2D高亮时候，需要的2D的object
        active_2d:()=>{

        },
    },
}

const vbw_block={
    hooks:self.hooks,
    transform:self.transform,  
    attribute:self.attribute,
}

export default vbw_block;